use ckb_std::ckb_types::{
    bytes::Bytes,
    packed::{RawTransaction, Script},
};
use serde::{Deserialize, Serialize};

pub trait UDT {
    type Error;
    fn balance() -> Result<u128, Self::Error>;
    fn transfer(
        tx: Option<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, Self::Error>;
}
pub const UDT_LEN: usize = 16;
pub enum UDTError {
    InsufficientBalance,
}

pub trait UDTMetadata: UDT {
    fn name() -> Result<Bytes, Self::Error>;
    fn symbol() -> Result<Bytes, Self::Error>;
    fn decimals() -> Result<u8, Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTMetadataData {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub extension_data_registry: Vec<UDTExtensionDataRegistry>,
}

// Note: This type is kept generic on purpose for future extensions.
#[derive(Serialize, Deserialize)]
pub struct UDTExtensionDataRegistry {
    pub registry_key: String,
    pub data: Bytes,
}

pub enum UDTMetadataError {
    NameUndefined,
    SymbolUndefined,
    DecimalsUndefined,
    TotalSupplyUndefined,
    CapUndefined,
    ExtensionDataNotFound,
}

pub trait UDTExtended: UDT + UDTMetadata {
    fn mint(
        tx: Option<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, Self::Error>;
    fn approve(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        amount: u128,
    ) -> Result<(), Self::Error>;
    fn allowance(owner: Script, spender_lock_hash: [u8; 32]) -> Result<u128, Self::Error>;
    fn increase_allowance(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        added_value: u128,
    ) -> Result<(), Self::Error>;
    fn decrease_allowance(
        tx: Option<RawTransaction>,
        spender_lock_hash: [u8; 32],
        subtracted_value: u128,
    ) -> Result<(), Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTExtendedData {}

pub enum UDTExtendedError {
    NoMintPermission,
    NoBurnPermission,
    NoApprovePermission,
    NoIncreaseAllowancePermission,
    NoDecreaseAllowancePermission,
}

pub trait UDTPausable: UDT + UDTMetadata {
    /* Note: Pausing/Unpause without lock hashes would take effect on the global level */
    fn pause(
        tx: Option<RawTransaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<RawTransaction, Self::Error>;
    fn unpause(
        tx: Option<RawTransaction>,
        lock_hashes: &Vec<[u8; 32]>,
    ) -> Result<RawTransaction, Self::Error>;
    fn is_paused(lock_hashes: &Vec<[u8; 32]>) -> Result<bool, Self::Error>;
    fn enumerate_paused() -> Result<Vec<[u8; 32]>, Self::Error>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTPausableData {
    pub pause_list: Vec<[u8; 32]>,
    pub next_type_hash: Option<[u8; 32]>,
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
