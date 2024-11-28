#![no_std]
extern crate alloc;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ckb_ssri_sdk::public_module_traits::udt::UDTPausableData;
use serde::{Deserialize, Serialize};
use serde_molecule::to_vec;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;

use syn::{parse_file, Fields, Item};

#[wasm_bindgen]
pub struct SSRITypeScriptOutput {
    molecule: String,
    interfaces: String
}

#[wasm_bindgen]
impl SSRITypeScriptOutput {
    #[wasm_bindgen(getter)]
    pub fn molecule(&self) -> String {
        self.molecule.clone()
    }
    #[wasm_bindgen(getter)]
    pub fn interfaces(&self) -> String {
        self.interfaces.clone()
    }
}

#[wasm_bindgen]
pub fn generate_typescript(source: &str) -> Result<SSRITypeScriptOutput, JsValue> {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();

    let syntax_tree = parse_file(source)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse Rust code: {}", e)))?;

    let mut output = String::new();

    // Add imports
    output.push_str("import { struct, option, vector } from \"@ckb-lumos/codec\";\n");
    output
        .push_str("import { Byte32, Uint8, BytesVec } from \"@ckb-lumos/base/lib/molecule\";\n\n");

    // Process each struct
    for item in syntax_tree.items {
        if let Item::Struct(item_struct) = item {
            if let Fields::Named(fields) = item_struct.fields {
                let struct_name = item_struct.ident.to_string();

                let mut field_defs = String::new();
                let mut field_names = Vec::new();

                for field in fields.named {
                    let field_name = field.ident.unwrap().to_string();
                    let field_type = convert_type_to_ts(&field.ty);
                    field_defs.push_str(&format!("    {}: {},\n", field_name, field_type));
                    field_names.push(format!("    \"{}\"", field_name));
                }

                output.push_str(&format!(
                    "export const {} = struct(\n  {{\n{}}},\n  [\n{}\n  ]\n);\n\n",
                    struct_name,
                    field_defs,
                    field_names.join(",\n")
                ));
            }
        }
    }

    Ok(GeneratorOutput { content: output })
}

fn rust_type_to_ts(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();
            let ident = segment.ident.to_string();
            
            match ident.as_str() {
                "Option" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let inner_type = rust_type_to_ts(
                            args.args.first().unwrap()
                                .into_value()
                                .as_type()
                                .unwrap()
                        );
                        format!("molecule.option({})", inner_type)
                    } else {
                        "molecule.option(molecule.table())".to_string()
                    }
                }
                "Vec" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let inner_type = rust_type_to_ts(
                            args.args.first().unwrap()
                                .into_value()
                                .as_type()
                                .unwrap()
                        );
                        format!("molecule.vector({})", inner_type)
                    } else {
                        "molecule.vector(molecule.table())".to_string()
                    }
                }
                "Transaction" => "Transaction.pack".to_string(),
                "Script" => "Script.pack".to_string(),
                "u128" => "number.Uint128.pack".to_string(),
                "Result" => {
                    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                        let ok_type = rust_type_to_ts(
                            args.args.first().unwrap()
                                .into_value()
                                .as_type()
                                .unwrap()
                        );
                        format!("molecule.result({})", ok_type)
                    } else {
                        "molecule.result(molecule.table())".to_string()
                    }
                }
                _ => "molecule.table()".to_string(),
            }
        }
        _ => "molecule.table()".to_string(),
    }
}


fn to_ts_interfaces();