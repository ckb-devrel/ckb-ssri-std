use crate::{
    error::Error,
    modules::PausableUDT,
    utils::{check_owner_mode, collect_inputs_amount, collect_outputs_amount},
};

use alloc::vec;
use alloc::vec::Vec;
use ckb_std::{
    ckb_constants::Source, ckb_types::{bytes::Bytes, prelude::*}, debug, high_level::{load_cell_lock_hash, load_script}
};

use ckb_ssri_sdk::public_module_traits::udt::{UDTPausable, UDT};

pub fn fallback() -> Result<(), Error> {
    debug!("Entered fallback");
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    if check_owner_mode(&args)? {
        return Ok(());
    }

    let inputs_amount = collect_inputs_amount()?;
    let outputs_amount = collect_outputs_amount()?;

    if inputs_amount < outputs_amount {
        return Err(Error::InsufficientBalance);
    }

    match PausableUDT::transfer(None, vec![], vec![]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
