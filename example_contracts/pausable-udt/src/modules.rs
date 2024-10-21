use crate::error::Error;
use crate::{get_metadata, get_pausable_data};
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ckb_ssri_sdk::public_module_traits::udt::{
    UDTExtended, UDTMetadata, UDTPausable, UDTPausableData, UDT,
};
use ckb_ssri_sdk::{ssri_contract, ssri_module};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::ckb_types::packed::{Byte32, RawTransaction, Script};
use ckb_std::high_level::{load_cell_data, load_cell_type_hash};
use serde_molecule::{from_slice, to_vec};

#[ssri_contract]
pub struct PausableUDT;

#[ssri_module]
impl UDT for PausableUDT {
    type Error = Error;
    #[ssri_method(level = "Cell")]
    fn balance() -> Result<u128, Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }
    #[ssri_method(level = "Transaction", transaction = true)]
    fn transfer(
        tx: Option<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, Error> {
        todo!()
    }
}

#[ssri_module(base = "UDT")]
impl UDTMetadata for PausableUDT {
    /** Note: If the UDT is issued with a generic UDT Type and defines it's metadata in CellDep, it would require Chain level; if it is only compliant to the SSRI trait UDT and is able to return name/symbol/decimals within the script, and it would require only code/script level. */
    #[ssri_method(level = "Code")]
    fn name() -> Result<Bytes, Error> {
        let metadata = get_metadata();
        Ok(Bytes::copy_from_slice(metadata.name.as_bytes()))
    }
    #[ssri_method(level = "Code")]
    fn symbol() -> Result<Bytes, Error> {
        let metadata = get_metadata();
        Ok(Bytes::copy_from_slice(metadata.symbol.as_bytes()))
    }
    #[ssri_method(level = "Code")]
    /* Note: By default, decimals are 8 when decimals() are not implemented */
    fn decimals() -> Result<u8, Error> {
        let metadata = get_metadata();
        Ok(metadata.decimals)
    }

    #[ssri_method(level = "Code")]
    fn get_extension_data(registry_key: String) -> Result<Bytes, Error> {
        let metadata = get_metadata();
        for extension_data in metadata.extension_data_registry {
            if extension_data.registry_key == registry_key {
                return Ok(extension_data.data);
            }
        }
        Err(Error::ExtensionDataNotFound)
    }
}

#[ssri_module(base = "UDT")]
impl UDTExtended for PausableUDT {
    #[ssri_method(level = "Transaction", transaction = true)]
    fn mint(
        tx: Option<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, Error> {
        todo!()
    }

    #[ssri_method(implemented = false)]
    fn approve(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        amount: u128,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    #[ssri_method(implemented = false)]
    fn allowance(owner: Script, spender_lock_hash: [u8; 32]) -> Result<u128, Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    #[ssri_method(implemented = false)]
    fn increase_allowance(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        added_value: u128,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    #[ssri_method(implemented = false)]
    fn decrease_allowance(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        subtracted_value: u128,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }
}

#[ssri_module(base = "UDT")]
impl UDTPausable for PausableUDT {
    #[ssri_method(level = "Transaction", transaction = true)]
    fn pause(
        tx: Option<RawTransaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<RawTransaction, Error> {
        todo!()
    }

    #[ssri_method(level = "Transaction", transaction = true)]
    fn unpause(
        tx: Option<RawTransaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<RawTransaction, Error> {
        todo!()
    }

    #[ssri_method(level = "Transaction", transaction = true)]
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Error> {
        let mut current_pause_list = Some(get_pausable_data().pause_list.clone()); // Start with an owned copy of the pause list
        
        while let Some(ref pause_list) = current_pause_list { // Borrow the pause list
            // Check the current pause list
            for lock_hash in lock_hashes {
                if pause_list.contains(lock_hash) {
                    return Ok(true);
                }
            }
            
            // Move to the next pause list if there is a next type hash
            match get_pausable_data().next_type_hash {
                Some(next_type_hash) => {
                    let mut index = 0;
                    while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                        if type_hash == Some(next_type_hash) {
                            let next_data: UDTPausableData =
                                from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                            current_pause_list = Some(next_data.pause_list); // Move ownership of the pause list
                            break; // Exit the loop to check the next pause list
                        }
                        index += 1;
                    }
                }
                None => {
                    return Ok(false); // No more pause lists, return false
                }
            }
        }
    
        Ok(false)
    }

    #[ssri_method(level = "Transaction", transaction = true)]
    fn enumerate_paused() -> Result<Vec<[u8; 32]>, Error> {
        let mut aggregated_paused_list: Vec<[u8; 32]> = vec![];
        aggregated_paused_list.extend(&get_pausable_data().pause_list.clone());
        
        while true {
            match get_pausable_data().next_type_hash {
                Some(next_type_hash) => {
                    let mut index = 0;
                    while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                        if type_hash == Some(next_type_hash) {
                            let next_data: UDTPausableData = from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                            aggregated_paused_list.extend(&next_data.pause_list);
                        }
                    }
                }
                None => {
                    return Ok(aggregated_paused_list); // Return the Vec by value
                }
            }
        }
        Ok(aggregated_paused_list)
    }
}
