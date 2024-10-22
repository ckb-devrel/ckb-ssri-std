#![no_std]

extern crate alloc;
extern crate proc_macro;

use core::panic;

use ckb_hash::blake2b_256;
use proc_macro::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::parse::ParseStream;
use syn::{parse_macro_input, Attribute, ImplItem, ItemFn, ItemImpl, ItemStruct, Meta};

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use darling::ast::NestedMeta;
use darling::{Error, FromAttributes, FromMeta};

// Struct to hold method metadata for reflection and dispatch
struct SSRIMethodMetadata {
    pub method_name: String,
    pub namespace: String,
    pub method_attributes: SSRIMethodAttributes,
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

#[derive(Debug, FromMeta)]
#[darling(default)]
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
    InvalidTraitName
}

// Function to extract the trait name (used as the namespace if `base` is not provided)
fn extract_trait_name(impl_block: &ItemImpl) -> Result<String, SSRISDKProcMacroError> {
    if let Some((_, path, _)) = &impl_block.trait_ {
        if let Some(segment) = path.segments.last() {
            return Ok(segment.ident.to_string());
        }
    }
    Err(SSRISDKProcMacroError::InvalidTraitName)
}

#[proc_macro_attribute]
pub fn ssri_method(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the method attributes using darling (SSRIMethodAttributes)
    let method_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let ssri_method_attributes: SSRIMethodAttributes =
        match SSRIMethodAttributes::from_list(&method_args) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };
    // Modify the method (e.g., add logging, check level, etc.)
    let method = parse_macro_input!(item as ItemFn);

    let method_metadata_const_name = format_ident!("__SSRIMETHOD_METADATA_{}", method.sig.ident);

    // Create a metadata struct to represent the parsed attributes
    let generated_method_metadata = quote! {
        const #method_metadata_const_name: SSRIMethodMetadata = SSRIMethodMetadata {
            namespace: "", // This will be set in ssri_module
            method_name: #method.sig.ident.to_string(),
            method_attributes: ssri_method_attributes
        };
    };

    // Return the method and the constant metadata
    let expanded = quote! {
        #method
        #generated_method_metadata
    };

    // Return the modified method as a TokenStream
    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn ssri_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    // Parse the attributes using `darling`'s `FromMeta` directly from the TokenStream
    let attr_args = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let ssri_module_attributes: SSRIModuleAttributes =
        match SSRIModuleAttributes::from_list(&attr_args) {
            Ok(v) => v,
            Err(e) => {
                return TokenStream::from(e.write_errors());
            }
        };

    let trait_name = extract_trait_name(&input);

    
    // Determine namespace based on `base` or fall back to the trait name
    let namespace = match ssri_module_attributes.base {
        Some(base) => base,
        None => match trait_name {
            Some(trait_name) => trait_name,
            None => return TokenStream::from(Error::custom("No trait name found").write_errors()),
        },
    };
    
    let module_metadata = Vec::new();

    let module_metadata_const_name = format_ident!("__SSRIMODULE_METADATA_{}", trait_name);


    for item in &impl_block.items {
        if let ImplItem::Const(const_item) = item {
            // We found a const generated by #[ssri_method]
            let const_ident = &const_item.ident;

            // Add the const to the metadata collection and fill in the namespace
            module_metadata.push(quote! {
                SSRIMethodMetadata {
                    namespace: #namespace,
                    ..#const_ident
                }
            });
        }
    }

    let generated_module_metadata = quote! {
        const #module_metadata_const_name: &[SSRIMethodMetadata] = &[#(#module_metadata),*];
    };


    let expanded = quote! {
        #input
        #generated_module_metadata
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
