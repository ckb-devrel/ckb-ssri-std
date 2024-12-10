//! Re-exports of procedural macros from ckb_ssri_sdk_proc_macro
//!
//! This module provides convenient access to all procedural macros defined in the
//! `ckb_ssri_sdk_proc_macro` crate. These macros are essential for implementing
//! SSRI-compliant smart contracts.
//!
//! # Available Macros
//!
//! - `ssri_module`: Marks a module as an SSRI-compliant module
//! - `ssri_method`: Marks a function as an exposed SSRI method
//!
//! # Example
//!
//! ```ignore
//! use ckb_ssri_sdk::macros::*;
//!
//! #[ssri_module]
//! mod my_contract {
//!     #[ssri_method]
//!     fn my_method() -> Result<(), Error> {
//!         // Implementation
//!     }
//! }
//! ```

pub use ckb_ssri_sdk_proc_macro::*;
