extern crate proc_macro;

use core::panic;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, ItemMod, Lit, Meta, MetaNameValue, NestedMeta};
pub use ckb_ssri_sdk_proc_macro::ssri_entry;
pub mod public_module_traits;

macro_rules! ssri_entry {
    ( $( $module:path ),* $(,)? ) => {
        pub fn unified_dispatch(namespace_and_function: &str, args: Vec<&str>) -> Result<String, crate::error::DispatchError> {
            $(
                let argv = ckb_std::env::argv();
                if argv.is_empty() {
                    return fallback::fallback().map(|_| ());
                }

                if vm_version() != u64::MAX {
                    return Err(Error::InvalidVmVersion);
                }

                set_content(&res)?;
                if $module::EXPOSED_FUNCTIONS.contains(&namespace_and_function) {
                    return $module::dispatch_function(namespace_and_function, args);
                }
            )*
            Err(crate::error::DispatchError::FunctionNotFound)
        }

        pub fn get_methods() -> Vec<&'static str> {
            let mut methods = Vec::new();
            $(
                methods.extend_from_slice($module::EXPOSED_FUNCTIONS);
            )*
            methods
        }

        pub fn 
    };
}

#[repr(i8)]
#[derive(Debug)]
pub enum SSRIError {
    SSRIMethodsNotFound = 4,
    SSRIMethodsArgsInvalid = 5,
    SSRIMethodsNotImplemented = 6,
}
