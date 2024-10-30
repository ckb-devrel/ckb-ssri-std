#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::borrow::Cow;
use ckb_ssri_sdk::prelude::{decode_u8_32_vector, encode_u8_32_vector};
use ckb_ssri_sdk_proc_macro::ssri_methods;
use ckb_std::debug;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();

use ckb_ssri_sdk::public_module_traits::udt::{
    UDTExtended, UDTMetadata, UDTMetadataData, UDTPausable, UDTPausableData, UDT,
};

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

mod error;
mod fallback;
mod modules;
mod syscall;
mod utils;

use ckb_std::syscalls::set_content;
use error::Error;
use syscall::vm_version;

pub fn get_metadata() -> UDTMetadataData {
    UDTMetadataData {
        name: String::from("UDT"),   // UDT name
        symbol: String::from("UDT"), // UDT symbol
        decimals: 8,
        extension_data_registry: vec![], // Extension data initialized empty for now
    }
}

pub fn get_pausable_data() -> UDTPausableData {
    debug!("Entered get_pausable_data");
    UDTPausableData {
        pause_list: utils::format_pause_list(vec![
            // Note: Paused lock hash for testing
            "0xd19228c64920eb8c3d79557d8ae59ee7a14b9d7de45ccf8bafacf82c91fc359e",
        ]),
        next_type_hash: None, // Type hash of cells that also contains UDTPausableData
    }
}

fn program_entry_wrap() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    debug!("argv: {:?}", argv);
    if argv.is_empty() {
        return Ok(fallback::fallback()?);
    }
    debug!("Entering ssri_methods");
    let res: Cow<'static, [u8]> = ssri_methods!(
        argv: &argv,
        invalid_method: Error::SSRIMethodsNotFound,
        invalid_args: Error::SSRIMethodsArgsInvalid,
        "SSRI.get_cell_deps" => Ok(Cow::from(&[0, 0, 0, 0][..])),
        "UDT.name" => Ok(Cow::from(modules::PausableUDT::name()?.to_vec())),
        "UDT.symbol" => Ok(Cow::from(modules::PausableUDT::symbol()?.to_vec())),
        "UDT.decimals" => Ok(Cow::from(modules::PausableUDT::decimals()?.to_le_bytes().to_vec())),
        "UDT.balance" => Ok(Cow::from(modules::PausableUDT::balance()?.to_le_bytes().to_vec())),
        "UDT.get_extension_data" => {
            let response = modules::PausableUDT::get_extension_data(String::from(argv[1].to_str()?))?;
            Ok(Cow::from(response.to_vec()))
        },
        "UDT.is_paused" => {
            let response = modules::PausableUDT::is_paused(&decode_u8_32_vector(decode_hex(argv[1].as_ref())?).map_err(|_|error::Error::InvalidArray)?)?;
            Ok(Cow::from(vec!(response as u8)))
        },
        "UDT.enumerate_paused" => {
            let response = encode_u8_32_vector(modules::PausableUDT::enumerate_paused()?.to_vec());
            Ok(Cow::from(response.to_vec()))
        },
    )?;

    set_content(&res)?;
    Ok(())
}

pub fn program_entry() -> i8 {
    match program_entry_wrap() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
