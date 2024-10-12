extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemMod, Lit, Meta, MetaNameValue, NestedMeta};

fn get_ssri_method_attrs(attrs: &[Attribute]) -> Option<(Option<String>, Option<bool>)> {
    for attr in attrs {
        if attr.path.is_ident("ssri") {
            // Parse the expose attribute parameters
            let meta = attr.parse_meta().ok()?;
            if let Meta::List(meta_list) = meta {
                /* Internal: Not exposed */
                let mut internal = None;
                /* Query: Require SSRI_VM */
                let mut query = None;

                for nested_meta in meta_list.nested {
                    if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                        ref path,
                        ref value,
                        ..
                    })) = nested_meta
                    {
                        // Handle the `rename` attribute
                        if path.is_ident("internal") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = value
                            {
                                internal = Some(lit_bool.value());
                            }
                        }
                        // Handle the `conditional` attribute
                        else if path.is_ident("conditional") {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = value
                            {
                                query = Some(lit_bool.value());
                            }
                        }
                    }
                }
                return Some((internal, query));
            }
            return Some((None, None)); // No parameters, just `#[expose]`
        }
    }
    None
}

fn get_ssri_module_attrs(attr: &Option<Attribute>) -> Option<Option<String>, Option<String>> {
    let mut version = None;
    let mut author = None;

    for nested_meta in attrs {
        if let NestedMeta::Meta(Meta::NameValue(MetaNameValue {
            path,
            value: Lit::Str(lit_str),
            ..
        })) = nested_meta
        {
            let key = path.get_ident().unwrap().to_string();
            match key.as_str() {
                "version" => version = Some(lit_str.value()),
                "author" => author = Some(lit_str.value()),
                _ => {}
            }
        }
    }

    (version, author)
}

#[proc_macro_attribute]
pub fn ssri_module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    let args = parse_macro_input!(attr as AttributeArgs);

    // Parse the module attributes (version, author, etc.)
    let (version, author) = get_ssri_module_attrs(args);

    let mod_name = &input.ident; // Module name (used as namespace)
    let mut function_signatures = Vec::new();
    let mut dispatch_cases = Vec::new();

    // Iterate over the items in the module (functions, etc.)
    if let Some((_, items)) = &input.content {
        for item in items {
            if let syn::Item::Fn(function) = item {
                // Check if the function has the #[expose] attribute and parse its parameters
                if let Some((rename, conditional)) = get_expose_attrs(&function.attrs) {
                    // If conditional is set to false, skip this function
                    if let Some(false) = conditional {
                        continue;
                    }

                    // Use the renamed function name for dispatch if provided
                    let function_name = rename.unwrap_or_else(|| function.sig.ident.to_string());

                    // Dynamically create argument identifiers (e.g., arg0, arg1, etc.)
                    let param_idents: Vec<_> = function
                        .sig
                        .inputs
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format_ident!("arg{}", i))
                        .collect();

                    // Collect parameters
                    let params: Vec<String> = function
                        .sig
                        .inputs
                        .iter()
                        .map(|arg| match arg {
                            syn::FnArg::Receiver(_) => "self".to_string(),
                            syn::FnArg::Typed(pat_type) => {
                                let ty = &pat_type.ty;
                                quote!(#ty).to_string()
                            }
                        })
                        .collect();

                    // Get the return type
                    let return_type = match &function.sig.output {
                        syn::ReturnType::Default => "void".to_string(),
                        syn::ReturnType::Type(_, ty) => quote!(#ty).to_string(),
                    };

                    // Create the signature string with metadata (version, author, etc.)
                    let signature = if let Some(ref ver) = version {
                        format!(
                            "{}::{}::{}({}) -> {} [version: {}, author: {}]",
                            mod_name,
                            ver,
                            function_name,
                            params.join(", "),
                            return_type,
                            ver,
                            author.clone().unwrap_or_else(|| "unknown".to_string())
                        )
                    } else {
                        format!(
                            "{}::{}({}) -> {} [author: {}]",
                            mod_name,
                            function_name,
                            params.join(", "),
                            return_type,
                            author.clone().unwrap_or_else(|| "unknown".to_string())
                        )
                    };
                    function_signatures.push(signature);

                    // Generate the match case for dispatch
                    let param_parsers: Vec<_> = function
                        .sig
                        .inputs
                        .iter()
                        .enumerate()
                        .map(|(i, arg)| {
                            match arg {
                                syn::FnArg::Receiver(_) => quote!(), // No need to parse `self`
                                syn::FnArg::Typed(pat_type) => {
                                    let ty = &pat_type.ty;
                                    let ident = &param_idents[i]; // Use the dynamically created identifier
                                    quote! {
                                        let #ident: #ty = args.get(#i)?.parse().ok()?;
                                    }
                                }
                            }
                        })
                        .collect();

                    let match_case = if let Some(ref ver) = version {
                        quote! {
                            concat!(stringify!(#mod_name), "::", #ver, "::", #function_name) => {
                                #(#param_parsers)*
                                Some(#function_ident(#(#param_idents),*).to_string())
                            }
                        }
                    } else {
                        quote! {
                            concat!(stringify!(#mod_name), "::", #function_name) => {
                                #(#param_parsers)*
                                Some(#function_ident(#(#param_idents),*).to_string())
                            }
                        }
                    };

                    dispatch_cases.push(match_case);
                }
            }
        }
    }

    // Generate the dispatch function and the list of function signatures for this module
    let expanded = quote! {
        #input

        pub fn version() -> u8;

        pub fn get_methods() -> Vec<&'static str> {
            vec![#(#function_signatures),*]
        }

        pub fn has_methods(function_signatures) -> Vec<bool> {
            todo!()
        }

        pub fn dispatch(namespace_and_function: &str, args: Vec<&str>) -> Option<String> {
            match namespace_and_function {
                #(#dispatch_cases,)*
                _ => None
            }
        }
    };

    TokenStream::from(expanded)
}
