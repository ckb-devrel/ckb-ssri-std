use crate::{
    error::Error,
    modules::UDTSSRI,
    utils::{check_owner_mode, collect_inputs_amount, collect_outputs_amount},
};

use alloc::vec;
use alloc::vec::Vec;
use ckb_std::high_level::{load_cell_lock_hash, load_script};

pub fn fallback() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();

    if check_owner_mode(&args)? {
        return Ok(());
    }

    let lock_hashes: Vec<[u8; 32]> = vec![];
    let mut index = 0;
    while let Ok(lock_hash) = load_cell_lock_hash(index, Source::Input) {
        lock_hashes.push(lock_hash);
        index += 1;
    }
    index = 0;
    while let Ok(lock_hash) = load_cell_lock_hash(index, Source::Output) {
        lock_hashes.push(lock_hash);
        index += 1;
    }

    if UDTSSRI::is_paused(&lock_hashes)? {
        return Err(Error::AbortedFromPause);
    }

    let inputs_amount = collect_inputs_amount()?;
    let outputs_amount = collect_outputs_amount()?;

    if inputs_amount < outputs_amount {
        return Err(Error::InsufficientBalance);
    }

    Ok(())
}
