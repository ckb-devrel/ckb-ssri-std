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
    ) -> Result<Option<Transaction>, Self::Error>;
}
pub const UDT_LEN: usize = 16;
pub enum UDTError {
    InsufficientBalance,
}

pub trait UDTMetadata: UDT {
    fn name() -> Result<Bytes, Self::Error>;
    fn symbol() -> Result<Bytes, Self::Error>;
    fn decimals() -> Result<u8, Self::Error>;
    fn get_extension_data(registry_key: String) -> Result<Bytes, Self::Error>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UDTMetadataData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    #[serde(with = "dynvec_serde")]
    pub extension_data_registry: Vec<UDTExtensionDataRegistry>,
}

// Note: This type is kept generic on purpose for future extensions.
#[derive(Serialize, Deserialize, Clone)]
pub struct UDTExtensionDataRegistry {
    pub registry_key: String,
    pub data: Vec<u8>,
}

pub enum UDTMetadataError {
    NameUndefined,
    SymbolUndefined,
    DecimalsUndefined,
    ExtensionDataNotFound,
}

pub trait UDTExtended: UDT + UDTMetadata {
    fn mint(
        tx: Option<Transaction>,
        to_lock_vec: Vec<Script>,
        to_amount_vec: Vec<u128>,
    ) -> Result<Option<Transaction>, Self::Error>;
    fn approve(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        amount: Option<u128>,
    ) -> Result<(), Self::Error>;
    fn allowance(owner: Script, spender_lock_hash: [u8; 32]) -> Result<u128, Self::Error>;
    fn increase_allowance(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        added_value: Option<u128>,
    ) -> Result<(), Self::Error>;
    fn decrease_allowance(
        tx: Option<Transaction>,
        spender_lock_hash: Option<[u8; 32]>,
        subtracted_value: Option<u128>,
    ) -> Result<(), Self::Error>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UDTExtendedData {}

pub enum UDTExtendedError {
    NoMintPermission,
    NoBurnPermission,
    NoApprovePermission,
    NoIncreaseAllowancePermission,
    NoDecreaseAllowancePermission,
}

pub trait UDTPausable: UDT + UDTMetadata {
    /* NOTE: Pausing/Unpause without lock hashes should take effect on the global level */
    fn pause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Self::Error>;
    fn unpause(
        tx: Option<Transaction>,
        lock_hashes: Option<&Vec<[u8; 32]>>,
    ) -> Result<Option<Transaction>, Self::Error>;
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Self::Error>;
    fn enumerate_paused() -> Result<Vec<UDTPausableData>, Self::Error>;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UDTPausableData {
    pub pause_list: Vec<[u8; 32]>,
    pub next_type_hash: Option<[u8; 32]>,
    pub next_type_args: Vec<u8>,
}

pub enum UDTPausableError {
    NoPausePermission,
    NoUnpausePermission,
    AbortedFromPause,
    IncompletePauseList,
}

pub enum UDTExtensionDataRegistryRecords {
    UDTPausableData,
    UDTExtendedData,
}
