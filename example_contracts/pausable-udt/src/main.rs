#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;
use alloc::borrow::Cow;
use alloc::vec;
use alloc::vec::Vec;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();
use ckb_ssri_sdk::{
    public_module_traits::{UDTMetadataData, UDTPausableData},
    ssri_entry, ssri_module,
};

mod error;
mod fallback;
mod modules;
mod syscall;
mod utils;

use error::Error;
use syscall::vm_version;

const METADATA: UDTMetadata = UDTMetadata {
    name: "UDT",   // UDT name
    symbol: "UDT", // UDT symbol
    decimals: 8,
};

const PAUSABLE_DATA: UDTPausableData = UDTPausableData {
    pause_list: vec![],
    next_type_hash: None, // Type hash of cells that also contains UDTPausableData
};

pub fn program_entry() -> i8 {
    ssri_entry!(fallback::fallback, [modules::PausableUDT])
}