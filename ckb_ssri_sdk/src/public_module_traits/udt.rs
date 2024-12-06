use ckb_std::ckb_types::{
    bytes::Bytes,
    packed::{Transaction, Script, Bytes as PackedBytes},
};
extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use serde::{Deserialize, Serialize};
use serde_molecule::dynvec_serde;

pub trait UDT {
    type Error;
    fn balance() -> Result<u128, Self::Error>;
    fn transfer(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Transaction, Self::Error>;
    fn verify_transfer() -> Result<(), Self::Error>;
    fn name() -> Result<Bytes, Self::Error>;
    fn symbol() -> Result<Bytes, Self::Error>;
    fn decimals() -> Result<u8, Self::Error>;
    fn mint(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Transaction, Self::Error>;
    fn verify_mint() -> Result<(), Self::Error>;
}
pub const UDT_LEN: usize = 16;
pub enum UDTError {
    InsufficientBalance,
    NoMintPermission,
    NoBurnPermission,
}

pub trait UDTPausable: UDT {
    /* NOTE: Pausing/Unpause without lock hashes should take effect on the global level */
    fn pause(
        tx: Option<Transaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<Transaction, Self::Error>;
    fn unpause(
        tx: Option<Transaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<Transaction, Self::Error>;
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Self::Error>;
    fn enumerate_paused() -> Result<Vec<UDTPausableData>, Self::Error>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UDTPausableData {
    pub pause_list: Vec<[u8; 32]>,
    pub next_type_script: Option<ScriptLike>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ScriptLike {
    pub code_hash: [u8; 32],
    pub hash_type: u8,
    pub args: Vec<u8>,
}
pub enum UDTPausableError {
    NoPausePermission,
    NoUnpausePermission,
    AbortedFromPause,
    IncompletePauseList,
    CyclicPauseList,
}
