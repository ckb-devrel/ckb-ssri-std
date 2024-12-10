#![no_std]
//! # CKB SSRI SDK
//! 
//! A comprehensive framework for implementing SSRI-compliant smart contracts on the Nervos CKB blockchain.
//! 
//! ## Overview
//! 
//! The SSRI (Standard Smart Contract Runtime Interface) SDK provides a standardized way to develop
//! smart contracts that are compliant with the SSRI protocol. This enables better interoperability
//! and a more consistent development experience across the CKB ecosystem.
//!
//! ## Features
//! 
//! - **Public Traits**: Pre-defined interfaces that receive first-class support within the ecosystem
//! - **Utility Functions**: Helper functions for SSRI-VM syscalls and data handling
//! - **Procedural Macros**: Simplify contract development with automatic SSRI method generation
//! - **No Standard Library**: Designed for the constrained smart contract environment
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ckb_ssri_sdk = "0.1.0"
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use ckb_ssri_sdk::prelude::*;
//! use ckb_ssri_sdk::public_module_traits::udt::UDT;
//! 
//! // Implement a basic UDT (User-Defined Token)
//! #[derive(Default)]
//! struct MyToken;
//! 
//! impl UDT for MyToken {
//!     type Error = ();
//!     // ... implement required methods
//! }
//! ```

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
///
/// This enum provides a standardized set of errors that can occur when executing
/// SSRI methods. These errors help identify issues with method discovery,
/// argument validation, implementation status, and environment compatibility.
///
/// # Examples
///
/// ```rust
/// use ckb_ssri_sdk::SSRIError;
///
/// fn example_handler() -> Result<(), SSRIError> {
///     // Method implementation missing
///     Err(SSRIError::SSRIMethodsNotImplemented)
/// }
/// ```
pub enum SSRIError {
    /// The requested SSRI method was not found in the contract
    SSRIMethodsNotFound,
    /// The arguments provided to the SSRI method were invalid
    SSRIMethodsArgsInvalid,
    /// The requested SSRI method is not implemented
    SSRIMethodsNotImplemented,
    /// The method requires a higher execution environment level
    SSRIMethodRequireHigherLevel,
    /// The CKB VM version is not compatible with this implementation
    InvalidVmVersion
}
