#![no_std]

extern crate alloc;
extern crate proc_macro;

use core::panic;

use ckb_hash::blake2b_256;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::ParseStream;
use syn::{parse_macro_input, Attribute, ItemFn, ItemImpl, ItemStruct, Meta};

use darling::ast::NestedMeta;
use darling::{Error, FromAttributes, FromMeta};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

// Struct to hold method metadata for reflection and dispatch
struct SSRIMethodMetadata {
    method_name: String,
    namespace: String,
    level: SSRIMethodLevel,
    transaction: bool,
}

#[derive(Debug)]
enum SSRIMethodLevel {
    Code,
    Script,
    Cell,
    Transaction,
}

impl Default for SSRIMethodLevel {
    fn default() -> Self {
        SSRIMethodLevel::Code
    }
}

impl FromMeta for SSRIMethodLevel {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
        match value {
            "Code" => Ok(SSRIMethodLevel::Code),
            "Script" => Ok(SSRIMethodLevel::Script),
            "Cell" => Ok(SSRIMethodLevel::Cell),
            "Transaction" => Ok(SSRIMethodLevel::Transaction),
            _ => Err(darling::Error::unknown_value(value)),
        }
    }
}

impl quote::ToTokens for SSRIMethodLevel {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let level_str = match self {
            SSRIMethodLevel::Code => "Code",
            SSRIMethodLevel::Script => "Script",
            SSRIMethodLevel::Cell => "Cell",
            SSRIMethodLevel::Transaction => "Transaction",
        };
        tokens.extend(quote! { #level_str });
    }
}

#[derive(Debug, FromAttributes)]
#[darling(default, attributes(ssri_method))]
struct SSRIMethodAttributes {
    #[darling(default)]
    pub implemented: bool,
    #[darling(default)]
    pub internal: bool,
    #[darling(default)]
    pub transaction: bool,
    #[darling(default)]
    pub level: SSRIMethodLevel,
}

impl Default for SSRIMethodAttributes {
    fn default() -> Self {
        SSRIMethodAttributes {
            implemented: true,
            internal: false,
            transaction: false,
            level: SSRIMethodLevel::Code,
        }
    }
}

#[derive(Debug, FromMeta)]
#[darling(default)]
struct SSRIModuleAttributes {
    #[darling(default)]
    pub version: String,

    #[darling(default)]
    pub base: Option<String>,
}

impl Default for SSRIModuleAttributes {
    fn default() -> Self {
        SSRIModuleAttributes {
            version: (&"0").to_string(),
            base: None,
        }
    }
}

#[derive(Debug)]
enum SSRISDKProcMacroError {
    InvalidMethodAttribute,
    InvalidModuleAttribute,
}

// Helper function to extract `Meta::List` items
fn extract_meta_list(attr: &Attribute) -> Vec<NestedMeta> {
    let mut attr_args = Vec::new();

    // Use `parse_args_with` to parse the tokens
    let _ = attr.parse_args_with(|input: syn::parse::ParseStream| {
        while !input.is_empty() {
            attr_args.push(input.parse()?);
        }
        Ok(())
    });

    attr_args
}

// Function to extract the trait name (used as the namespace if `base` is not provided)
fn extract_trait_name(impl_block: &ItemImpl) -> Option<String> {
    if let Some((_, path, _)) = &impl_block.trait_ {
        if let Some(segment) = path.segments.last() {
            return Some(segment.ident.to_string());
        }
    }
    None
}

// Function to parse method-level attributes using `darling`
fn parse_ssri_method_attributes(attrs: &[syn::Attribute]) -> Result<SSRIMethodAttributes, darling::Error> {
    SSRIMethodAttributes::from_attributes(attrs)
}

#[proc_macro_attribute]
pub fn ssri_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    // Parse the attributes using `darling`'s `FromMeta` directly from the TokenStream
    let ssri_module_attributes = match SSRIModuleAttributes::from_meta(&syn::parse_macro_input!(attr as syn::Meta)) {
        Ok(attrs) => attrs,
        Err(err) => return TokenStream::from(err.write_errors()),
    };


    // Determine namespace based on `base` or fall back to the trait name
    let namespace = match ssri_module_attributes.base {
        Some(base) => base,
        None => match extract_trait_name(&input) {
            Some(trait_name) => trait_name,
            None => return TokenStream::from(Error::custom("No trait name found").write_errors()),
        },
    };

    let mut method_metadata = Vec::new();

    // Collect methods annotated with #[ssri_method] and gather metadata
    for item in &input.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = method.sig.ident.to_string();

            // Parse ssri_method attributes for this method
            let ssri_method_attributes = match parse_ssri_method_attributes(&method.attrs) {
                Ok(attrs) => attrs,
                Err(err) => return TokenStream::from(err.write_errors()),
            };

            let level = ssri_method_attributes.level;
            let transaction = ssri_method_attributes.transaction;

            method_metadata.push(SSRIMethodMetadata {
                method_name,
                namespace: namespace.clone(),
                level,
                transaction,
            });
        }
    }

    // Pass this metadata through the generated code so ssri_contract can collect it
    let method_metadata_tokens = method_metadata.iter().map(|meta| {
        let method_name = &meta.method_name;
        let namespace = &meta.namespace;
        let level = &meta.level;
        let transaction = meta.transaction;

        quote! {
            SSRIMethodMetadata {
                method_name: #method_name.to_string(),
                namespace: #namespace.to_string(),
                level: #level.to_string(),
                transaction: #transaction,
            }
        }
    });

    let expanded = quote! {
        #input

        pub const SSRI_METHODS: &[SSRIMethodMetadata] = &[#(#method_metadata_tokens),*];
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn ssri_contract(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct); // Parse the struct with ssri_contract

    let struct_name = &input.ident;

    // Collect all SSRI_METHODS from the ssri_module implementations
    let method_collection_tokens = quote! {
        let methods: &[&[SSRIMethodMetadata]] = &[SSRI_METHODS];
    };

    // Generate the dispatch logic dynamically based on the collected methods
    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn version() -> u8 {
                0
            }

            pub fn get_methods() -> Vec<&'static str> {
                #method_collection_tokens

                let mut method_names = Vec::new();
                for methods_in_module in methods.iter() {
                    for method in *methods_in_module {
                        method_names.push(method.method_name);
                    }
                }
                method_names
            }

            pub fn has_methods(function_signatures: Vec<&str>) -> Vec<bool> {
                #method_collection_tokens

                function_signatures.iter().map(|f| {
                    methods.iter().any(|methods_in_module| {
                        methods_in_module.iter().any(|method| {
                            *f == method.method_name
                        })
                    })
                }).collect()
            }

            pub fn dispatch(namespace_and_function: &str, args: Vec<&str>) -> Option<String> {
                #method_collection_tokens

                for methods_in_module in methods.iter() {
                    for method in *methods_in_module {
                        let full_method_name = format!("{}.{}", method.namespace, method.method_name);
                        if full_method_name == namespace_and_function {
                            // Implement the actual logic to call the correct method here
                            return Some(full_method_name); // Replace with actual dispatch call
                        }
                    }
                }
                None
            }
        }
    };

    TokenStream::from(expanded)
}
