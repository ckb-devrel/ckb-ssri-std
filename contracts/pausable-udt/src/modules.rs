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
use ckb_std::ckb_types::core::ScriptHashType;
use ckb_std::ckb_types::packed::{
    Byte as PackedByte, Byte32Vec, BytesVec, BytesVecBuilder, CellDepVec, CellInputVecBuilder,
    CellOutput, CellOutputBuilder, CellOutputVecBuilder, RawTransactionBuilder, Script,
    ScriptOptBuilder, Transaction, TransactionBuilder, Uint32, Uint64,
};
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::debug;
use ckb_std::high_level::{
    decode_hex, load_cell, load_cell_data, load_cell_lock_hash, load_cell_type_hash, load_input,
    load_script, load_transaction,
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
            let inputs_amount = collect_inputs_amount()?;
            let outputs_amount = collect_outputs_amount()?;

            if inputs_amount < outputs_amount {
                return Err(Error::InsufficientBalance);
            }
            debug!("inputs_amount: {}", inputs_amount);
            debug!("outputs_amount: {}", outputs_amount);
            Ok((None))
        } else {
            if to_amount_vec.len() != to_lock_vec.len() {
                return Err(Error::SSRIMethodsArgsInvalid);
            }
            let tx_builder = match tx {
                Some(ref tx) => tx.clone().as_builder(),
                None => TransactionBuilder::default(),
            };
            let raw_tx_builder = match tx {
                Some(ref tx) => tx.clone().raw().as_builder(),
                None => RawTransactionBuilder::default(),
            };

            let cell_output_vec = match tx {
                Some(ref tx) => {
                    let mut cell_output_vec_builder = tx.clone().raw().outputs().as_builder();
                    for to_lock in to_lock_vec.iter() {
                        let new_transfer_output = CellOutputBuilder::default()
                            .type_(
                                ScriptOptBuilder::default()
                                    .set(Some(load_script()?))
                                    .build(),
                            )
                            .capacity(Uint64::default())
                            .lock(to_lock.clone())
                            .build();
                        debug!("New transfer output: {:?}", new_transfer_output);
                        cell_output_vec_builder = cell_output_vec_builder.push(new_transfer_output);
                    }
                    cell_output_vec_builder.build()
                }
                None => {
                    let mut cell_output_vec_builder = CellOutputVecBuilder::default();
                    for to_lock in to_lock_vec.iter() {
                        let new_transfer_output = CellOutputBuilder::default()
                            .type_(
                                ScriptOptBuilder::default()
                                    .set(Some(load_script()?))
                                    .build(),
                            )
                            .capacity(Uint64::default())
                            .lock(to_lock.clone())
                            .build();
                        cell_output_vec_builder = cell_output_vec_builder.push(new_transfer_output);
                    }
                    cell_output_vec_builder.build()
                }
            };
            let outputs_data = match tx {
                Some(ref tx) => {
                    let mut tx_builder = tx.clone().raw().outputs_data().as_builder();
                    for to_amount in to_amount_vec.iter() {
                        tx_builder = tx_builder.push(to_amount.pack().as_bytes().pack());
                    }
                    tx_builder.build()
                }
                None => {
                    let mut tx_builder = BytesVecBuilder::default();
                    for to_amount in to_amount_vec.iter() {
                        tx_builder = tx_builder.push(to_amount.pack().as_bytes().pack());
                    }
                    tx_builder.build()
                }
            };
            let response = tx_builder
                .raw(
                    raw_tx_builder
                        .version(tx.clone().should_be_ok().raw().version())
                        .cell_deps(tx.clone().should_be_ok().raw().cell_deps())
                        .header_deps(tx.clone().should_be_ok().raw().header_deps())
                        .inputs(tx.clone().should_be_ok().raw().inputs())
                        .outputs(cell_output_vec)
                        .outputs_data(outputs_data)
                        .build(),
                )
                .witnesses(tx.clone().should_be_ok().witnesses())
                .build();
            Ok(Some(response))
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
        if should_fallback()? {
            return Err(Error::SSRIMethodsArgsInvalid);
        }
        debug!("Entered UDT::mint");
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

        let mut new_cell_output_vec: Vec<CellOutput> = Vec::new();
        let mut new_output_data_vec = Vec::new();
        for (to_lock, to_amount) in to_lock_vec.iter().zip(to_amount_vec.iter()) {
            let new_transfer_output = CellOutputBuilder::default()
                .type_(
                    ScriptOptBuilder::default()
                        .set(Some(load_script()?))
                        .build(),
                )
                .lock(to_lock.clone())
                .build();
            new_cell_output_vec.push(new_transfer_output);
            new_output_data_vec.push(to_amount);
        }

        let cell_output_vec_builder = match tx {
            Some(ref tx) => {
                let mut builder = tx.clone().raw().outputs().as_builder();
                for output in new_cell_output_vec {
                    builder = builder.push(output);
                }
                builder
            }
            None => {
                let mut builder = CellOutputVecBuilder::default();
                for output in new_cell_output_vec {
                    builder = builder.push(output);
                }
                builder
            }
        };

        let output_data_vec_builder = match tx {
            Some(ref tx) => {
                let mut builder = tx.clone().raw().outputs_data().as_builder();
                for data in new_output_data_vec {
                    builder = builder.push(data.pack().as_bytes().pack());
                }
                builder
            }
            None => {
                let mut builder = BytesVecBuilder::default();
                for data in new_output_data_vec {
                    builder = builder.push(data.pack().as_bytes().pack());
                }
                builder
            }
        };

        Ok(Some(
            tx_builder
                .raw(
                    raw_tx_builder
                        .version(tx.clone().should_be_ok().raw().version())
                        .cell_deps(tx.clone().should_be_ok().raw().cell_deps())
                        .header_deps(tx.clone().should_be_ok().raw().header_deps())
                        .inputs(tx.clone().should_be_ok().raw().inputs())
                        .outputs(cell_output_vec_builder.build())
                        .outputs_data(output_data_vec_builder.build())
                        .build(),
                )
                .build(),
        ))
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
    // #[ssri_method(level = "cell", transaction = true)]
    fn pause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Error> {
        if should_fallback()? {
            return Err(Error::SSRIMethodsArgsInvalid);
        }
        if lock_hashes.is_none() {
            return Err(Error::SSRIMethodsArgsInvalid);
        }
        let tx_builder = match tx {
            Some(ref tx) => tx.clone().as_builder(),
            None => TransactionBuilder::default(),
        };
        let raw_tx_builder = match tx {
            Some(ref tx) => tx.clone().raw().as_builder(),
            None => RawTransactionBuilder::default(),
        };
        let mut current_pausable_data = get_pausable_data();
        let mut index = 0;
        while let Some(next_type_hash) = current_pausable_data.next_type_hash {
            let mut found = false;
            index = 0;

            while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                if type_hash == Some(next_type_hash) {
                    current_pausable_data =
                        from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                    found = true;
                    break;
                }
                index += 1;
            }

            if !found {
                let cell_output_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().outputs().as_builder();
                        builder = builder.push(load_cell(0, Source::Input)?);
                        builder
                    }
                    None => {
                        let mut builder = CellOutputVecBuilder::default();
                        builder = builder.push(load_cell(0, Source::Input)?);
                        builder
                    }
                };
                let new_output_data = match tx {
                    Some(ref tx) => {
                        let mut data: UDTPausableData =
                            from_slice(&load_cell_data(0, Source::Input)?, true)?;
                        data.pause_list.extend(lock_hashes.should_be_ok().clone());
                        data
                    }
                    None => UDTPausableData {
                        pause_list: lock_hashes.should_be_ok().clone(),
                        next_type_hash: None,
                    },
                };

                let output_data_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().outputs_data().as_builder();
                        builder = builder.push(to_vec(&new_output_data, false)?.pack());
                        builder
                    }
                    None => {
                        let mut builder = BytesVecBuilder::default();
                        builder = builder.push(to_vec(&new_output_data, false)?.pack());
                        builder
                    }
                };

                let cell_input_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().inputs().as_builder();
                        builder = builder.push(load_input(0, Source::Input)?);
                        builder
                    }
                    None => {
                        let mut builder = CellInputVecBuilder::default();
                        builder = builder.push(load_input(0, Source::Input)?);
                        builder
                    }
                };
                return Ok(Some(
                    tx_builder
                        .raw(
                            raw_tx_builder
                                .version(Uint32::default())
                                .cell_deps(CellDepVec::default())
                                .header_deps(Byte32Vec::default())
                                .inputs(cell_input_vec_builder.build())
                                .outputs(cell_output_vec_builder.build())
                                .outputs_data(output_data_vec_builder.build())
                                .build(),
                        )
                        .witnesses(BytesVec::default())
                        .build(),
                ));
            }
        }
        Err(Error::SSRIMethodsArgsInvalid)
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn unpause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Error> {
        if should_fallback()? {
            return Err(Error::SSRIMethodsArgsInvalid);
        }
        if lock_hashes.is_none() {
            return Err(Error::SSRIMethodsArgsInvalid);
        }
        let tx_builder = match tx {
            Some(ref tx) => tx.clone().as_builder(),
            None => TransactionBuilder::default(),
        };
        let raw_tx_builder = match tx {
            Some(ref tx) => tx.clone().raw().as_builder(),
            None => RawTransactionBuilder::default(),
        };
        let mut current_pausable_data = get_pausable_data();
        let mut index = 0;
        while let Some(next_type_hash) = current_pausable_data.next_type_hash {
            let mut found = false;
            index = 0;

            while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                if type_hash == Some(next_type_hash) {
                    current_pausable_data =
                        from_slice(&load_cell_data(index, Source::CellDep)?, false)?;
                    found = true;
                    break;
                }
                index += 1;
            }

            if !found {
                let cell_output_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().outputs().as_builder();
                        builder = builder.push(load_cell(0, Source::Input)?);
                        builder
                    }
                    None => {
                        let mut builder = CellOutputVecBuilder::default();
                        builder = builder.push(load_cell(0, Source::Input)?);
                        builder
                    }
                };
                let new_output_data = match tx {
                    Some(ref tx) => {
                        let mut data: UDTPausableData =
                            from_slice(&load_cell_data(0, Source::Input)?, true)?;
                        // If lock hash is in pause list, remove it
                        data.pause_list
                            .retain(|x| !lock_hashes.should_be_ok().contains(x));
                        data
                    }
                    None => {
                        let mut data: UDTPausableData =
                            from_slice(&load_cell_data(0, Source::Input)?, true)?;
                        data.pause_list
                            .retain(|x| !lock_hashes.should_be_ok().contains(x));
                        data
                    }
                };

                let output_data_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().outputs_data().as_builder();
                        builder = builder.push(to_vec(&new_output_data, false)?.pack());
                        builder
                    }
                    None => {
                        let mut builder = BytesVecBuilder::default();
                        builder = builder.push(to_vec(&new_output_data, false)?.pack());
                        builder
                    }
                };

                let cell_input_vec_builder = match tx {
                    Some(ref tx) => {
                        let mut builder = tx.clone().raw().inputs().as_builder();
                        builder = builder.push(load_input(0, Source::Input)?);
                        builder
                    }
                    None => {
                        let mut builder = CellInputVecBuilder::default();
                        builder = builder.push(load_input(0, Source::Input)?);
                        builder
                    }
                };
                return Ok(Some(
                    tx_builder
                        .raw(
                            raw_tx_builder
                                .inputs(cell_input_vec_builder.build())
                                .outputs(cell_output_vec_builder.build())
                                .outputs_data(output_data_vec_builder.build())
                                .build(),
                        )
                        .build(),
                ));
            }
        }
        Err(Error::SSRIMethodsArgsInvalid)
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Error> {
        debug!("Entered is_paused");
        debug!("lock_hashes: {:?}", lock_hashes);
        let mut current_pausable_data = get_pausable_data();
        // Return true if any of the lock hashes are in the pause list
        if lock_hashes
            .iter()
            .any(|x| current_pausable_data.pause_list.contains(x))
        {
            return Ok(true);
        }
        let mut seen_type_hashes: Vec<[u8; 32]> = Vec::new();

        while let Some(next_type_hash) = current_pausable_data.next_type_hash {
            // Detect cycles
            if !seen_type_hashes
                .clone()
                .into_iter()
                .any(|x| x == next_type_hash)
            {
                return Err(Error::CyclicPauseList)?;
            } else {
                seen_type_hashes.push(next_type_hash);
            }

            // Find next node
            let mut index = 0;
            let mut found = false;

            while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                if type_hash == Some(next_type_hash) {
                    match load_cell_data(index, Source::CellDep) {
                        Ok(data) => {
                            current_pausable_data = from_slice(&data, false)?;
                            // Return true if any of the lock hashes are in the pause list
                            if lock_hashes
                                .iter()
                                .any(|x| current_pausable_data.pause_list.contains(x))
                            {
                                return Ok(true);
                            }
                            found = true;
                            break;
                        }
                        Err(e) => return Err(Error::InvalidPauseData),
                    }
                }
                index += 1;
            }

            if !found {
                return Err(Error::IncompletePauseList)?;
            }
        }
        Ok(false)
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn enumerate_paused() -> Result<Vec<[u8; 32]>, Error> {
        let mut aggregated_paused_list: Vec<[u8; 32]> = Vec::new();
        let mut current_pausable_data = get_pausable_data();
        let mut seen_type_hashes: Vec<[u8; 32]> = Vec::new();

        // Add initial pause list
        aggregated_paused_list.extend(&current_pausable_data.pause_list);

        while let Some(next_type_hash) = current_pausable_data.next_type_hash {
            // Detect cycles
            if !seen_type_hashes
                .clone()
                .into_iter()
                .any(|x| x == next_type_hash)
            {
                return Err(Error::CyclicPauseList)?;
            } else {
                seen_type_hashes.push(next_type_hash);
            }

            // Find next node
            let mut index = 0;
            let mut found = false;

            while let Ok(type_hash) = load_cell_type_hash(index, Source::CellDep) {
                if type_hash == Some(next_type_hash) {
                    match load_cell_data(index, Source::CellDep) {
                        Ok(data) => {
                            current_pausable_data = from_slice(&data, false)?;
                            aggregated_paused_list.extend(&current_pausable_data.pause_list);
                            found = true;
                            break;
                        }
                        Err(e) => return Err(Error::InvalidPauseData),
                    }
                }
                index += 1;
            }

            if !found {
                return Err(Error::IncompletePauseList)?;
            }
        }

        Ok(aggregated_paused_list)
    }
}
