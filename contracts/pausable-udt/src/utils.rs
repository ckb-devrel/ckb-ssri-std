use alloc::vec::Vec;
use ckb_std::{ckb_constants::Source, ckb_types::bytes::Bytes, high_level::{load_cell_data, load_cell_lock_hash, QueryIter}};
use ckb_ssri_sdk::public_module_traits::udt::UDT_LEN;
use crate::error::Error;

pub fn collect_inputs_amount() -> Result<u128, Error> {
    // let's loop through all input cells containing current UDTs,
    // and gather the sum of all input tokens.
    let mut buf = [0u8; UDT_LEN];

    let udt_list = QueryIter::new(load_cell_data, Source::GroupInput)
        .map(|data| {
            if data.len() == UDT_LEN {
                buf.copy_from_slice(&data);
                // u128 is 16 bytes
                Ok(u128::from_le_bytes(buf))
            } else {
                Err(Error::Encoding)
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}

pub fn collect_outputs_amount() -> Result<u128, Error> {
    // With the sum of all input UDT tokens gathered, let's now iterate through
    // output cells to grab the sum of all output UDT tokens.
    let mut buf = [0u8; UDT_LEN];

    let udt_list = QueryIter::new(load_cell_data, Source::GroupOutput)
        .map(|data| {
            if data.len() == UDT_LEN {
                buf.copy_from_slice(&data);
                // u128 is 16 bytes
                Ok(u128::from_le_bytes(buf))
            } else {
                Err(Error::Encoding)
            }
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(udt_list.into_iter().sum::<u128>())
}

pub fn check_owner_mode(args: &Bytes) -> Result<bool, Error> {
    // With owner lock script extracted, we will look through each input in the
    // current transaction to see if any unlocked cell uses owner lock.
    let is_owner_mode = QueryIter::new(load_cell_lock_hash, Source::Input)
        .find(|lock_hash| args[..] == lock_hash[..])
        .is_some();
    Ok(is_owner_mode)
}
