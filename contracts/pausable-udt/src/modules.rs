use crate::error::Error;
use crate::utils::{collect_inputs_amount, collect_outputs_amount};
use crate::{get_metadata, get_pausable_data};
use alloc::ffi::CString;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ckb_ssri_sdk::public_module_traits::udt::{
    UDTExtended, UDTMetadata, UDTPausable, UDTPausableData, UDT,
};
use ckb_ssri_sdk::utils::should_fallback;
// use ckb_ssri_sdk_proc_macro::{ssri_method, ssri_module};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::bytes::Bytes;
use ckb_std::ckb_types::core::ScriptHashType;
use ckb_std::ckb_types::packed::{
    Byte32, CellOutput, CellOutputBuilder, CellOutputVecBuilder, RawTransactionBuilder, Script,
    ScriptBuilder, ScriptOpt, ScriptOptBuilder, Transaction, TransactionBuilder,
};
use ckb_std::ckb_types::prelude::{Builder, Entity, Pack, ShouldBeOk, Unpack};
use ckb_std::debug;
use ckb_std::high_level::{
    decode_hex, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_script,
    load_transaction,
};
use serde_molecule::{from_slice, to_vec};

pub struct PausableUDT;

// #[ssri_module]
impl UDT for PausableUDT {
    type Error = Error;
    // #[ssri_method(level = "Cell")]
    fn balance() -> Result<u128, Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }
    // #[ssri_method(level = "Transaction", transaction = true)]
    fn transfer(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Option<Transaction>, Error> {
        debug!("Entered UDT::transfer");
        if should_fallback()? {
            let script = load_script()?;
            let args: Bytes = script.args().unpack();

            let mut lock_hashes: Vec<[u8; 32]> = vec![];

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

            if PausableUDT::is_paused(&lock_hashes)? {
                return Err(Error::AbortedFromPause);
            }

            let inputs_amount = collect_inputs_amount()?;
            let outputs_amount = collect_outputs_amount()?;

            if inputs_amount < outputs_amount {
                return Err(Error::InsufficientBalance);
            }
            debug!("inputs_amount: {}", inputs_amount);
            debug!("outputs_amount: {}", outputs_amount);
            Ok((None))
        } else {
            let tx_builder = match tx {
                Some(ref tx) => tx.clone().as_builder(),
                None => TransactionBuilder::default(),
            };
            let raw_tx_builder = match tx {
                Some(ref tx) => tx.clone().raw().as_builder(),
                None => RawTransactionBuilder::default(),
            };

            let cell_output_vec_builder = match tx {
                Some(ref tx) => tx.clone().raw().outputs().as_builder(),
                None => CellOutputVecBuilder::default(),
            };

            let script = ScriptBuilder::default()
                .code_hash(
                    Byte32::from_compatible_slice(
                        decode_hex(
                            CString::new(
                                "64e62e0f847240e23bea2801af6c39a62be25b7dce522cef3462624fa260135e",
                            )
                            .unwrap()
                            .as_c_str(),
                        )?
                        .as_slice(),
                    )
                    .unwrap(),
                )
                .args(
                    Bytes::from_static(
                        decode_hex(
                            CString::new(
                                "b5202efa0f2d250af66f0f571e4b8be8b272572663707a052907f8760112fe35",
                            )
                            .unwrap()
                            .as_c_str(),
                        )?
                        .as_slice(),
                    )
                    .pack(),
                )
                .hash_type(ScriptHashType::Type.into())
                .build();

            let new_transfer_output = CellOutputBuilder::default()
                // TODO: Set lock and capacity automatically in CCC.
                .type_(
                    ScriptOptBuilder::default()
                        // TODO: load_script under run_script_level_script fails.
                        // .set(Some(load_script()?))
                        .set(Some(script))
                        .build(),
                )
                .build();

            Ok(Some(
                tx_builder
                    .raw(
                        raw_tx_builder
                            .outputs(cell_output_vec_builder.push(new_transfer_output).build())
                            .build(),
                    )
                    .build(),
            ))
        }
    }
}

// #[ssri_module(base = "UDT")]
impl UDTMetadata for PausableUDT {
    // #[ssri_method(level = "Code")]
    fn name() -> Result<Bytes, Error> {
        let metadata = get_metadata();
        Ok(Bytes::from(metadata.name.into_bytes()))
    }
    // #[ssri_method(level = "Code")]
    fn symbol() -> Result<Bytes, Error> {
        let metadata = get_metadata();
        Ok(Bytes::from(metadata.symbol.into_bytes()))
    }
    // #[ssri_method(level = "Code")]
    fn decimals() -> Result<u8, Error> {
        let metadata = get_metadata().clone();
        Ok(metadata.decimals)
    }

    // #[ssri_method(level = "Code")]
    fn get_extension_data(registry_key: String) -> Result<Bytes, Error> {
        let metadata = get_metadata();
        for extension_data in metadata.extension_data_registry {
            if extension_data.registry_key == registry_key {
                return Ok(Bytes::from(extension_data.data));
            }
        }
        Err(Error::ExtensionDataNotFound)
    }
}

// #[ssri_module(base = "UDT")]
impl UDTExtended for PausableUDT {
    // #[ssri_method(level = "Transaction", transaction = true)]
    fn mint(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Option<Transaction>, Error> {
        todo!()
    }

    // #[ssri_method(implemented = false)]
    fn approve(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        amount: Option<u128>,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    // #[ssri_method(implemented = false)]
    fn allowance(owner: Script, spender_lock_hash: [u8; 32]) -> Result<u128, Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    // #[ssri_method(implemented = false)]
    fn increase_allowance(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        added_value: Option<u128>,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }

    // #[ssri_method(implemented = false)]
    fn decrease_allowance(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        subtracted_value: Option<u128>,
    ) -> Result<(), Error> {
        Err(Error::SSRIMethodsNotImplemented)
    }
}

// #[ssri_module(base = "UDT")]
impl UDTPausable for PausableUDT {
    // #[ssri_method(level = "Transaction", transaction = true)]
    fn pause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Error> {
        todo!()
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn unpause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Error> {
        todo!()
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Error> {
        debug!("Entered is_paused");
        debug!("lock_hashes: {:?}", lock_hashes);
        let mut current_pause_list = Some(get_pausable_data().pause_list.clone()); // Start with an owned copy of the pause list

        while let Some(ref pause_list) = current_pause_list {
            debug!("Current pause list: {:?}", pause_list);
            if pause_list.contains(&[0u8; 32]) {
                debug!("Global pausing found");
                return Ok(true);
            }
            for lock_hash in lock_hashes {
                if pause_list.contains(lock_hash) {
                    debug!("Lock hash found in pause list");
                    return Ok(true);
                }
            }

            match get_pausable_data().next_type_hash {
                Some(next_type_hash) => {
                    let mut index = 0;
                    while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                        if type_hash == Some(next_type_hash) {
                            let next_data: UDTPausableData =
                                from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                            current_pause_list = Some(next_data.pause_list);
                            break;
                        }
                        index += 1;
                    }
                }
                None => {
                    debug!("No more pause lists and lock hash not found");
                    return Ok(false);
                }
            }
        }
        debug!("No pause list found and lock hash not found");
        Ok(false)
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn enumerate_paused() -> Result<Vec<[u8; 32]>, Error> {
        let mut aggregated_paused_list: Vec<[u8; 32]> = vec![];
        aggregated_paused_list.extend(&get_pausable_data().pause_list.clone());

        while true {
            match get_pausable_data().next_type_hash {
                Some(next_type_hash) => {
                    let mut index = 0;
                    while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                        if type_hash == Some(next_type_hash) {
                            let next_data: UDTPausableData =
                                from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                            aggregated_paused_list.extend(&next_data.pause_list);
                        }
                    }
                }
                None => {
                    return Ok(aggregated_paused_list);
                }
            }
        }
        Ok(aggregated_paused_list)
    }
}
