use crate::error::Error;
use crate::get_pausable_data;
use crate::utils::{check_owner_mode, collect_inputs_amount, collect_outputs_amount};
use alloc::string::String;
use alloc::vec::Vec;
use ckb_ssri_sdk::public_module_traits::udt::{UDTPausable, UDTPausableData, UDT};
use ckb_ssri_sdk::utils::high_level::{
    find_cell_by_out_point, find_cell_data_by_out_point, find_out_point_by_type,
};
// use ckb_ssri_sdk_proc_macro::{ssri_method, ssri_module};
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::packed::{
    Byte, Byte32, BytesVec, BytesVecBuilder, CellOutput, CellOutputBuilder, CellOutputVecBuilder,
    RawTransactionBuilder, Script, ScriptBuilder, ScriptOptBuilder, Transaction,
    TransactionBuilder, Uint64,
};
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::debug;
use ckb_std::high_level::{load_cell, load_cell_data, load_script};
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
    ) -> Result<Transaction, Error> {
        debug!("Entered UDT::transfer");
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

        let mut cell_output_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs().as_builder(),
            None => CellOutputVecBuilder::default(),
        };

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

        let mut outputs_data_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs_data().as_builder(),
            None => BytesVecBuilder::default(),
        };

        for to_amount in to_amount_vec.iter() {
            outputs_data_builder = outputs_data_builder.push(to_amount.pack().as_bytes().pack());
        }
        Ok(tx_builder
            .raw(
                raw_tx_builder
                    .version(tx.clone().should_be_ok().raw().version())
                    .cell_deps(tx.clone().should_be_ok().raw().cell_deps())
                    .header_deps(tx.clone().should_be_ok().raw().header_deps())
                    .inputs(tx.clone().should_be_ok().raw().inputs())
                    .outputs(cell_output_vec_builder.build())
                    .outputs_data(outputs_data_builder.build())
                    .build(),
            )
            .witnesses(tx.clone().should_be_ok().witnesses())
            .build())
    }

    fn verify_transfer() -> Result<(), Self::Error> {
        let inputs_amount = collect_inputs_amount()?;
        let outputs_amount = collect_outputs_amount()?;

        if inputs_amount < outputs_amount {
            return Err(Error::InsufficientBalance);
        }
        debug!("inputs_amount: {}", inputs_amount);
        debug!("outputs_amount: {}", outputs_amount);
        Ok(())
    }

    fn name() -> Result<Bytes, Self::Error> {
        Ok(Bytes::from(String::from("PUDT").into_bytes()))
    }

    fn symbol() -> Result<Bytes, Self::Error> {
        Ok(Bytes::from(String::from("PUDT").into_bytes()))
    }

    fn decimals() -> Result<u8, Self::Error> {
        Ok(8u8)
    }

    fn mint(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Transaction, Error> {
        debug!("Entered UDT::mint");
        let tx_builder = match tx {
            Some(ref tx) => tx.clone().as_builder(),
            None => TransactionBuilder::default(),
        };
        let raw_tx_builder = match tx {
            Some(ref tx) => tx.clone().raw().as_builder(),
            None => RawTransactionBuilder::default(),
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

        let mut cell_output_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs().as_builder(),
            None => CellOutputVecBuilder::default(),
        };

        for output in new_cell_output_vec {
            cell_output_vec_builder = cell_output_vec_builder.push(output);
        }

        let mut output_data_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs_data().as_builder(),
            None => BytesVecBuilder::default(),
        };
        for data in new_output_data_vec {
            output_data_vec_builder = output_data_vec_builder.push(data.pack().as_bytes().pack());
        }

        Ok(tx_builder
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
            .build())
    }

    fn verify_mint() -> Result<(), Self::Error> {
        let script = load_script()?;
        let args: Bytes = script.args().unpack();
        if check_owner_mode(&args)? {
            return Ok(());
        } else {
            return Err(Error::NoMintPermission);
        }
    }
}

// #[ssri_module(base = "UDT")]
impl UDTPausable for PausableUDT {
    // #[ssri_method(level = "cell", transaction = true)]
    fn pause(tx: Option<Transaction>, lock_hashes: &Vec<[u8; 32]>) -> Result<Transaction, Error> {
        let tx_builder = match tx {
            Some(ref tx) => tx.clone().as_builder(),
            None => TransactionBuilder::default(),
        };
        let raw_tx_builder = match tx {
            Some(ref tx) => tx.clone().raw().as_builder(),
            None => RawTransactionBuilder::default(),
        };
        let pausable_data_vec: Vec<UDTPausableData> = Self::enumerate_paused(0, 0)?;

        let new_cell_output: CellOutput;
        let new_output_data: UDTPausableData;
        match load_cell(0, Source::Input) {
            Ok(cell_output) => {
                new_cell_output = cell_output;
                let cell_data = load_cell_data(0, Source::Input)?;
                let mut pausable_data: UDTPausableData = from_slice(&cell_data, false)?;
                pausable_data.pause_list.extend(lock_hashes.clone());
                new_output_data = pausable_data;
            }
            Err(_) => {
                if pausable_data_vec.len() < 2 {
                    new_cell_output = CellOutput::default();
                    new_output_data = UDTPausableData {
                        pause_list: lock_hashes.clone(),
                        next_type_script: None,
                    };
                } else {
                    let second_last_pausable_data = pausable_data_vec
                        .get(pausable_data_vec.len() - 2)
                        .should_be_ok();
                    let last_cell_type_script = ScriptBuilder::default()
                        .code_hash(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .code_hash
                                .pack(),
                        )
                        .hash_type(Byte::new(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .hash_type,
                        ))
                        .args(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .args
                                .pack(),
                        )
                        .build();
                    let last_cell_out_point = find_out_point_by_type(last_cell_type_script)?;
                    new_cell_output = find_cell_by_out_point(last_cell_out_point.clone())?;
                    let last_cell_data = find_cell_data_by_out_point(last_cell_out_point)?;
                    let mut pausable_data: UDTPausableData = from_slice(&last_cell_data, false)?;
                    pausable_data.pause_list.extend(lock_hashes.clone());
                    new_output_data = pausable_data;
                }
            }
        };

        let cell_output_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs().as_builder(),
            None => CellOutputVecBuilder::default(),
        }
        .push(new_cell_output);

        let output_data_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs_data().as_builder(),
            None => BytesVecBuilder::default(),
        }
        .push(to_vec(&new_output_data, false)?.pack());

        return Ok(tx_builder
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
            .witnesses(BytesVec::default())
            .build());
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn unpause(tx: Option<Transaction>, lock_hashes: &Vec<[u8; 32]>) -> Result<Transaction, Error> {
        let tx_builder = match tx {
            Some(ref tx) => tx.clone().as_builder(),
            None => TransactionBuilder::default(),
        };
        let raw_tx_builder = match tx {
            Some(ref tx) => tx.clone().raw().as_builder(),
            None => RawTransactionBuilder::default(),
        };
        let pausable_data_vec: Vec<UDTPausableData> = Self::enumerate_paused(0, 0)?;

        let new_cell_output: CellOutput;
        let new_output_data: UDTPausableData;
        match load_cell(0, Source::Input) {
            Ok(cell_output) => {
                new_cell_output = cell_output;
                let cell_data = load_cell_data(0, Source::Input)?;
                let mut pausable_data: UDTPausableData = from_slice(&cell_data, false)?;
                pausable_data.pause_list = pausable_data
                    .pause_list
                    .into_iter()
                    .filter(|x| !lock_hashes.contains(x))
                    .collect();
                new_output_data = pausable_data;
            }
            Err(_) => {
                if pausable_data_vec.len() < 2 {
                    return Err(Error::SSRIMethodsArgsInvalid);
                } else {
                    let second_last_pausable_data = pausable_data_vec
                        .get(pausable_data_vec.len() - 2)
                        .should_be_ok();
                    let last_cell_type_script = ScriptBuilder::default()
                        .code_hash(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .code_hash
                                .pack(),
                        )
                        .hash_type(Byte::new(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .hash_type,
                        ))
                        .args(
                            second_last_pausable_data
                                .clone()
                                .next_type_script
                                .should_be_ok()
                                .args
                                .pack(),
                        )
                        .build();
                    let last_cell_out_point = find_out_point_by_type(last_cell_type_script)?;
                    new_cell_output = find_cell_by_out_point(last_cell_out_point.clone())?;
                    let last_cell_data = find_cell_data_by_out_point(last_cell_out_point)?;
                    let mut pausable_data: UDTPausableData = from_slice(&last_cell_data, false)?;
                    pausable_data.pause_list = pausable_data
                        .pause_list
                        .into_iter()
                        .filter(|x| !lock_hashes.contains(x))
                        .collect();
                    new_output_data = pausable_data;
                }
            }
        };

        let cell_output_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs().as_builder(),
            None => CellOutputVecBuilder::default(),
        }
        .push(new_cell_output);

        let output_data_vec_builder = match tx {
            Some(ref tx) => tx.clone().raw().outputs_data().as_builder(),
            None => BytesVecBuilder::default(),
        }
        .push(to_vec(&new_output_data, false)?.pack());

        return Ok(tx_builder
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
            .witnesses(BytesVec::default())
            .build());
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Error> {
        debug!("Entered is_paused");
        debug!("lock_hashes: {:?}", lock_hashes);

        let pausable_data_vec: Vec<UDTPausableData> = Self::enumerate_paused(0, 0)?;
        for pausable_data in pausable_data_vec {
            if pausable_data
                .pause_list
                .into_iter()
                .any(|x| lock_hashes.contains(&x))
            {
                return Ok(true);
            }
        }
        Ok(false)
    }

    // #[ssri_method(level = "Transaction", transaction = true)]
    fn enumerate_paused(mut offset: u64, limit: u64) -> Result<Vec<UDTPausableData>, Error> {
        let mut pausable_data_vec: Vec<UDTPausableData> = Vec::new();
        let mut current_pausable_data = get_pausable_data()?;
        let mut seen_type_hashes: Vec<Byte32> = Vec::new();
        let mut entries_counter = 0;

        // Handle initial data
        if current_pausable_data.pause_list.len() < offset as usize {
            offset -= current_pausable_data.pause_list.len() as u64;
        } else {
            let mut modified_data = current_pausable_data.clone();
            modified_data.pause_list = modified_data
                .pause_list
                .into_iter()
                .skip(offset as usize)
                .collect();
            if limit != 0 && modified_data.pause_list.len() as u64 > limit {
                modified_data.pause_list = modified_data
                    .pause_list
                    .into_iter()
                    .take(limit as usize)
                    .collect();
            }
            entries_counter += modified_data.pause_list.len() as u64;
            if entries_counter > 0 {
                pausable_data_vec.push(modified_data);
            }
            offset = 0;
            if limit != 0 && entries_counter >= limit {
                return Ok(pausable_data_vec);
            }
        }

        while let Some(next_type_script) = current_pausable_data.next_type_script {
            let next_type_script: Script = Script::new_builder()
                .code_hash(next_type_script.code_hash.pack())
                .hash_type(Byte::new(next_type_script.hash_type))
                .args(next_type_script.args.pack())
                .build();
            let mut next_pausable_data: UDTPausableData = from_slice(
                &find_cell_data_by_out_point(find_out_point_by_type(next_type_script.clone())?)?,
                false,
            )?;

            if seen_type_hashes
                .clone()
                .into_iter()
                .any(|x| x == next_type_script.calc_script_hash())
            {
                return Err(Error::CyclicPauseList)?;
            } else {
                seen_type_hashes.push(next_type_script.calc_script_hash());
            }

            if next_pausable_data.pause_list.len() < offset as usize {
                offset -= next_pausable_data.pause_list.len() as u64;
                current_pausable_data = next_pausable_data;
            } else {
                next_pausable_data.pause_list = next_pausable_data
                    .pause_list
                    .into_iter()
                    .skip(offset as usize)
                    .collect();
                pausable_data_vec.push(next_pausable_data.clone());
                entries_counter += next_pausable_data.pause_list.len() as u64;
                offset = 0;
                if limit != 0 && entries_counter >= limit {
                    break;
                }
                current_pausable_data = next_pausable_data;
            }
        }

        if entries_counter > limit && limit != 0 {
            if let Some(last) = pausable_data_vec.last_mut() {
                last.pause_list = last
                    .pause_list
                    .clone()
                    .into_iter()
                    .take(last.pause_list.len() - (entries_counter - limit) as usize)
                    .collect();
            }
        }

        Ok(pausable_data_vec)
    }
}
