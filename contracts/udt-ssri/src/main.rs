#![no_std]
#![cfg_attr(not(test), no_main)]

#[cfg(test)]
extern crate alloc;

use alloc::borrow::Cow;
#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!();
use ckb_ssri_sdk::{ssri_entry, ssri_module};

mod error;
mod fallback;
mod syscall;
mod modules;

use error::Error;
use syscall::vm_version;

pub fn program_entry() -> i8 {
    ssri_entry!(fallback::fallback, [
        modules::UDTSSRI,
    ])
}

fn program_entry_wrap() -> Result<(), Error> {
    Ok(())
}
