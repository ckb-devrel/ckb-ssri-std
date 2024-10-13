use crate::SSRIError;
use serde::{Serialize, Deserialize};

pub trait UDT {
    fn balance() -> Result<u128, SSRIError>;
    fn transfer(to: Script, amount: u128) -> Result<(), SSRIError>;
}

pub enum UDTError {
    InsufficientBalance,
}

pub trait UDTMetadata: UDT {
    fn name() -> Result<Bytes, SSRIError>;
    fn symbol() -> Result<Bytes, SSRIError>;
    fn decimals() -> Result<u8, SSRIError>;
    fn total_supply() -> Result<u128, SSRIError>;
    fn cap() -> Result<u128, SSRIError>;
}
#[derive(Serialize, Deserialize)]
pub struct UDTMetadataData {
    name: String,
    symbol: String,
    decimals: u8,
    total_supply: u128,
    cap: u128,
    extension_data_registry: UDTExtensionDataRegistry
}

#[derive(Serialize, Deserialize)]
pub struct UDTExtensionDataRegistry {
    #[serde(with = "dynvec_serde")]
    extended_data: Vec<UDTExtendedData>,
    #[serde(with = "dynvec_serde")]
    pausable_data: Vec<UDTPausableData>,
}

pub enum UDTMetadataError {
    NameUndefined,
    SymbolUndefined,
    DecimalsUndefined,
    TotalSupplyUndefined,
    CapUndefined,
    ExtensionDataNotFound,
}

pub trait UDTExtended: UDT {
    fn balance_of(lock: Script) -> Result<u128, SSRIError>;
    fn transfer(from: Script, to: Script, amount: u128) -> Result<(), SSRIError>;
    fn mint(lock: Script, amount: u128) -> Result<(), SSRIError>;
    fn burn(lock: Script, amount: u128) -> Result<(), SSRIError>;
    fn approve(spender: Script, amount: u128) -> Result<(), SSRIError>;
    fn allowance(owner: Script, spender: Script) -> Result<u128, SSRIError>;
    fn increase_allowance(spender: Script, added_value: u128) -> Result<(), SSRIError>;
    fn decrease_allowance(spender: Script, subtracted_value: u128) -> Result<(), SSRIError>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTExtendedData {
}

pub enum UDTExtendedError {
    NoTransferPermission,
    NoMintPermission,
    NoBurnPermission,
    NoApprovePermission,
    NoIncreaseAllowancePermission,
    NoDecreaseAllowancePermission,
}

pub trait UDTPausable: UDT {
    /* Note: Pausing/Unpause without lock would take effect on the global level */
    fn pause(lock: Option<Script>) -> Result<(), SSRIError>;
    fn unpause(lock: Option<Script>) -> Result<(), SSRIError>;
}

pub struct UDTPausableData {
    pause_list: Vec<Byte32>,
    next_type_hash: Byte32,
}

pub enum UDTPausableError {
    NoPausePermission,
    NoUnpausePermission,
    AbortedFromPause
}




