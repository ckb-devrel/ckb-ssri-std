#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::borrow::Cow;
use ckb_ssri_sdk::prelude::{decode_u8_32_vector, encode_u8_32_vector};

use ckb_ssri_sdk::utils::should_fallback;
use ckb_ssri_sdk_proc_macro::ssri_methods;
use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::ckb_types::packed::{Byte32, BytesVec, Script, ScriptBuilder, Transaction};
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
        extension_data_registry: vec![
            UDTExtensionDataRegistry {
                registry_key: String::from("UDTPausableData"),
                data: to_vec(&get_pausable_data(), true).unwrap(),
            },
        ], // Store data in an external UDTMetadataData cell for greater flexibility in configuring your UDT.
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
            let response = modules::PausableUDT::is_paused(&decode_u8_32_vector(decode_hex(argv[1].as_ref())?).map_err(|_|error::Error::SSRIMethodsArgsInvalid)?)?;
            Ok(Cow::from(vec!(response as u8)))
        },
        "UDTPausable.enumerate_paused" => {
            let response = encode_u8_32_vector(modules::PausableUDT::enumerate_paused()?.to_vec());
            Ok(Cow::from(response.to_vec()))
        },
        "UDT.transfer" => {
            debug!("program_entry_wrap | Entered UDT.transfer");
            let to_lock_bytes_vec = BytesVec::new_unchecked(decode_hex(argv[2].as_ref())?.try_into().unwrap());
            let to_lock_vec: Vec<Script> = to_lock_bytes_vec
                .into_iter()
                .map(|bytes| Script::new_unchecked(bytes.unpack()))
                .collect();
            debug!("program_entry_wrap | to_lock_vec: {:?}", to_lock_vec);

            let to_amount_bytes = decode_hex(argv[3].as_ref())?;
            let to_amount_vec: Vec<u128> = decode_hex(argv[3].as_ref())?[4..]
                .chunks(16)
                .map(|chunk| {
                    return u128::from_le_bytes(chunk.try_into().unwrap())}
                )
                .collect();
            debug!("program_entry_wrap | to_amount_vec: {:?}", to_amount_vec);

            if argv[2].is_empty() || argv[3].is_empty() || to_lock_vec.len() != to_amount_vec.len() {
                Err(Error::SSRIMethodsArgsInvalid)?;
            }

            let mut tx: Option<Transaction> = None;
            if argv[1].is_empty() {
                tx = None;
            } else {
                let parsed_tx: Transaction = Transaction::from_compatible_slice(&Bytes::from_static(&decode_hex(argv[1].as_ref())?)).map_err(|_|Error::MoleculeVerificationError)?;
                tx = Some(parsed_tx);
            }

            let response_opt = modules::PausableUDT::transfer(tx, to_lock_vec, to_amount_vec)?;
            match response_opt {
                Some(response) => {
                    debug!("program_entry_wrap | response: {}", response);
                    Ok(Cow::from(response.as_bytes().to_vec()))
                },
                None => Err(Error::SSRIMethodsArgsInvalid),
            }
        },
        "UDTExtended.mint" => {
            debug!("program_entry_wrap | Entered UDTExtended.mint");
            let to_lock_bytes_vec = BytesVec::new_unchecked(decode_hex(argv[2].as_ref())?.try_into().unwrap());
            let to_lock_vec: Vec<Script> = to_lock_bytes_vec
                .into_iter()
                .map(|bytes| Script::new_unchecked(bytes.unpack()))
                .collect();
            debug!("program_entry_wrap | to_lock_vec: {:?}", to_lock_vec);

            let to_amount_bytes = decode_hex(argv[3].as_ref())?;
            let to_amount_vec: Vec<u128> = decode_hex(argv[3].as_ref())?[4..]
                .chunks(16)
                .map(|chunk| {
                    return u128::from_le_bytes(chunk.try_into().unwrap())}
                )
                .collect();
            debug!("program_entry_wrap | to_amount_vec: {:?}", to_amount_vec);

            if argv[2].is_empty() || argv[3].is_empty() || to_lock_vec.len() != to_amount_vec.len() {
                Err(Error::SSRIMethodsArgsInvalid)?;
            }

            let mut tx: Option<Transaction> = None;
            if argv[1].is_empty() {
                tx = None;
            } else {
                let parsed_tx: Transaction = Transaction::new_unchecked(Bytes::from_static(&decode_hex(argv[1].as_ref())?));
                tx = Some(parsed_tx);
            }

            let response_opt = modules::PausableUDT::transfer(tx, to_lock_vec, to_amount_vec)?;
            match response_opt {
                Some(response) => Ok(Cow::from(response.as_bytes().to_vec())),
                None => Err(Error::SSRIMethodsArgsInvalid),
            }
        },
        "UDTPausable.pause" => {
            debug!("program_entry_wrap | Entered UDTPausable.pause");
            let lock_hashes_vec: Vec<[u8; 32]> = decode_u8_32_vector(decode_hex(argv[2].as_ref())?).map_err(|_|error::Error::InvalidArray)?;
            debug!("program_entry_wrap | lock_hashes_vec: {:?}", lock_hashes_vec);

            if argv[2].is_empty() {
                Err(Error::SSRIMethodsArgsInvalid)?;
            }

            let mut tx: Option<Transaction> = None;
            if argv[1].is_empty() {
                tx = None;
            } else {
                let parsed_tx: Transaction = Transaction::new_unchecked(Bytes::from_static(&decode_hex(argv[1].as_ref())?));
                tx = Some(parsed_tx);
            }

            let response_opt = modules::PausableUDT::pause(tx, Some(&lock_hashes_vec))?;
            match response_opt {
                Some(response) => Ok(Cow::from(response.as_bytes().to_vec())),
                None => Err(Error::SSRIMethodsArgsInvalid),
            }
        },
        "UDTPausable.unpause" => {
            debug!("program_entry_wrap | Entered UDTPausable.unpause");
            let lock_hashes_vec: Vec<[u8; 32]> = decode_u8_32_vector(decode_hex(argv[2].as_ref())?).map_err(|_|error::Error::InvalidArray)?;
            debug!("program_entry_wrap | lock_hashes_vec: {:?}", lock_hashes_vec);

            if argv[2].is_empty() {
                Err(Error::SSRIMethodsArgsInvalid)?;
            }

            let mut tx: Option<Transaction> = None;
            if argv[1].is_empty() {
                tx = None;
            } else {
                let parsed_tx: Transaction = Transaction::from_compatible_slice(&Bytes::from_static(&decode_hex(argv[1].as_ref())?)).map_err(|_|Error::MoleculeVerificationError)?;
                tx = Some(parsed_tx);
            }

            let response_opt = modules::PausableUDT::unpause(tx, Some(&lock_hashes_vec))?;
            match response_opt {
                Some(response) => Ok(Cow::from(response.as_bytes().to_vec())),
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
