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

mod error;
mod fallback;
mod syscall;
mod udt;

use ckb_ssri_proc_macro::ssri_methods;
use ckb_std::syscalls::set_content;
use error::Error;
use syscall::vm_version;

pub fn program_entry() -> i8 {
    match program_entry_wrap() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}

fn program_entry_wrap() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    if argv.is_empty() {
        return fallback::fallback().map(|_| ());
    }

    if vm_version() != u64::MAX {
        return Err(Error::InvalidVmVersion);
    }

    set_content(&res)?;

    Ok(())
}
