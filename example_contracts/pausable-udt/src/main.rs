#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;
use alloc::{borrow::Cow, string::ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::String;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();
use ckb_ssri_sdk::{
    public_module_traits::udt::{UDTMetadataData, UDTMetadataData, UDTPausableData},
    ssri_entry, ssri_module,
};

mod error;
mod fallback;
mod modules;
mod syscall;
mod utils;

use error::Error;
use syscall::vm_version;

pub fn get_metadata() -> UDTMetadataData {
    UDTMetadataData {
        name: String::from("UDT"), // UDT name
        symbol: String::from("UDT"), // UDT symbol
        decimals: 8,
        extension_data_registry: vec![], // Extension data initialized empty for now
    }
}

pub fn get_pausable_data() -> UDTPausableData {
    UDTPausableData {
        pause_list: vec![],
        next_type_hash: None, // Type hash of cells that also contains UDTPausableData
    }
}

pub fn program_entry() -> i8 {
    ssri_entry!(fallback::fallback, [modules::PausableUDT])
}