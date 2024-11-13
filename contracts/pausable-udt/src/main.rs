#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::borrow::Cow;
use ckb_ssri_sdk::prelude::{decode_u8_32_vector, encode_u8_32_vector};

use ckb_ssri_sdk::utils::should_fallback;
use ckb_ssri_sdk_proc_macro::ssri_methods;
use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::ckb_types::packed::{Byte32, Script, ScriptBuilder, Transaction};
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

use ckb_std::ckb_types::prelude::{Builder, Entity, ShouldBeOk, Unpack};

mod ckb_types_serde_molecule;
mod error;
mod fallback;
mod modules;
mod utils;

use ckb_std::syscalls::set_content;
use error::Error;
use serde_molecule::{from_slice, to_vec};

pub fn get_metadata() -> UDTMetadataData {
    UDTMetadataData {
        name: String::from("UDT"),
        symbol: String::from("UDT"),
        decimals: 8,
        extension_data_registry: vec![], // Store data in an external UDTMetadataData cell for greater flexibility in configuring your UDT.
    }
}

pub fn get_pausable_data() -> UDTPausableData {
    debug!("Entered get_pausable_data");
    UDTPausableData {
        pause_list: utils::format_pause_list(vec![
            // Note: Paused lock hash for testing for ckb_ssri_cli. The address is ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqgtlcnzzna2tqst7jw78egjpujn7hdxpackjmmdp
            "0xd19228c64920eb8c3d79557d8ae59ee7a14b9d7de45ccf8bafacf82c91fc359e",
        ]),
        next_type_hash: None, // Type hash of another cell that also contains UDTPausableData
    }
}

fn program_entry_wrap() -> Result<(), Error> {
    let argv = ckb_std::env::argv();

    if should_fallback()? {
        return Ok(fallback::fallback()?);
    }

    debug!("Entering ssri_methods");
    // NOTE: In the future, methods can be reflected automatically from traits using procedural macros and entry methods to other methods of the same trait for a more concise and maintainable entry function.
    let res: Cow<'static, [u8]> = ssri_methods!(
        argv: &argv,
        invalid_method: Error::SSRIMethodsNotFound,
        invalid_args: Error::SSRIMethodsArgsInvalid,
        "SSRI.get_cell_deps" => Ok(Cow::from(&[0, 0, 0, 0][..])),
        "UDTMetadata.name" => Ok(Cow::from(modules::PausableUDT::name()?.to_vec())),
        "UDTMetadata.symbol" => Ok(Cow::from(modules::PausableUDT::symbol()?.to_vec())),
        "UDTMetadata.decimals" => Ok(Cow::from(modules::PausableUDT::decimals()?.to_le_bytes().to_vec())),
        "UDT.balance" => Ok(Cow::from(modules::PausableUDT::balance()?.to_le_bytes().to_vec())),
        "UDTMetadata.get_extension_data" => {
            let response = modules::PausableUDT::get_extension_data(String::from(argv[1].to_str()?))?;
            Ok(Cow::from(response.to_vec()))
        },
        "UDTPausable.is_paused" => {
            let response = modules::PausableUDT::is_paused(&decode_u8_32_vector(decode_hex(argv[1].as_ref())?).map_err(|_|error::Error::InvalidArray)?)?;
            Ok(Cow::from(vec!(response as u8)))
        },
        "UDTPausable.enumerate_paused" => {
            let response = encode_u8_32_vector(modules::PausableUDT::enumerate_paused()?.to_vec());
            Ok(Cow::from(response.to_vec()))
        },
        "UDT.transfer" => {
            let mut tx: Option<Transaction> = None;
            let mut to: Option<Vec<(Script, u128)>> = None;
            if argv[1].is_empty() {
                tx = None;
            } else {
                let parsed_tx: Transaction = Transaction::new_unchecked(Bytes::from_static(&decode_hex(argv[1].as_ref())?));
                tx = Some(parsed_tx);
            }
            if argv[2].is_empty(){
                Err(Error::SSRIMethodsArgsInvalid)?;
            } else {
                let parsed_to: Vec<(ckb_types_serde_molecule::Script, u128)> = from_slice(&decode_hex(argv[1].as_ref())?, false)?;
                let mut typed_to: Vec<(Script, u128)> = vec![];
                for (script, amount) in parsed_to.iter() {
                    let typed_script = ScriptBuilder::default()
                        .code_hash(Byte32::new_unchecked(Bytes::from_static(&script.code_hash)))
                        .hash_type(script.hash_type.into())
                        .args(ckb_std::ckb_types::packed::Bytes::new_unchecked(script.args.as_slice().into()))
                        .build();
                    typed_to.push((typed_script, *amount));
                }
                to = Some(typed_to)
            }
            if tx.is_none() || to.is_none() {
                Err(Error::SSRIMethodsArgsInvalid)?;
            }
            let response_opt = modules::PausableUDT::transfer(tx, to)?;
            match response_opt {
                Some(response) => Ok(Cow::from(response.as_slice().to_vec())),
                None => Err(Error::SSRIMethodsArgsInvalid),
            }
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
