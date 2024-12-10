#![no_std]
//! CKB SSRI SDK - A framework for implementing SSRI-compliant smart contracts
//! 
//! This crate provides the core functionality for building SSRI-compliant smart contracts
//! on the Nervos CKB blockchain. It includes traits, utilities and procedural macros to
//! simplify contract development following the SSRI protocol.
//!
//! # Features
//! 
//! - Pre-defined traits for common contract patterns
//! - Utilities for CKB syscalls and data handling
//! - Procedural macros for SSRI method generation
//! - No-std environment support for CKB contracts

// use quote::quote;
// use syn::{parse_macro_input, Attribute, ItemMod, Lit, Meta, MetaNameValue};

pub mod public_module_traits;
pub mod prelude;
pub mod utils;
pub mod macros;

// Re-export proc macros at crate root for convenience
pub use macros::*;

extern crate alloc;

// macro_rules! ssri_entry {
//     ( $( $module:path ),* $(,)? ) => {
//         pub fn unified_dispatch(namespace_and_function: &str, args: Vec<&str>) -> Result<String, crate::error::DispatchError> {
//             $(
//                 let argv = ckb_std::env::argv();
//                 if argv.is_empty() {
//                     return fallback::fallback().map(|_| ());
//                 }

//                 if vm_version() != u64::MAX {
//                     return Err(Error::InvalidVmVersion);
//                 }

//                 set_content(&res)?;
//                 if $module::EXPOSED_FUNCTIONS.contains(&namespace_and_function) {
//                     return $module::dispatch_function(namespace_and_function, args);
//                 }
//             )*
//             Err(crate::error::DispatchError::FunctionNotFound)
//         }

//         pub fn get_methods() -> Vec<&'static str> {
//             let mut methods = Vec::new();
//             $(
//                 methods.extend_from_slice($module::EXPOSED_FUNCTIONS);
//             )*
//             methods
//         }
//     };
// }

#[repr(i8)]
#[derive(Debug)]
/// Represents possible errors that can occur during SSRI method execution
pub enum SSRIError {
    /// The requested SSRI method was not found in the contract
    SSRIMethodsNotFound,
    /// The arguments provided to the SSRI method were invalid
    SSRIMethodsArgsInvalid,
    /// The requested SSRI method is not implemented
    SSRIMethodsNotImplemented,
    /// The method requires a higher execution environment level
    SSRIMethodRequireHigherLevel,
    /// The CKB VM version is not compatible
    InvalidVmVersion
}
