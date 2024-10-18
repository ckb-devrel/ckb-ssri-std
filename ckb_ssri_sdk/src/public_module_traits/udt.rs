pub trait UDT {
    fn balance() -> Result<u128, SSRIError>;
    fn transfer(
        tx: Optional<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, SSRIError>;
}

pub enum UDTError {
    InsufficientBalance,
}

pub trait UDTMetadata: UDT {
    fn name() -> Result<Bytes, SSRIError>;
    fn symbol() -> Result<Bytes, SSRIError>;
    fn decimals() -> Result<u8, SSRIError>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTMetadataData {
    name: String,
    symbol: String,
    decimals: u8,
    extension_data_registry: Vec<UDTExtensionDataRegistry>,
}

// Note: This type is kept generic on purpose for future extensions.
#[derive(Serialize, Deserialize)]
pub struct UDTExtensionDataRegistry {
    registry_key: String,
    data: Bytes,
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
        tx: Optional<RawTransaction>,
        to: Vec<(Script, u128)>,
    ) -> Result<RawTransaction, SSRIError>;
    fn approve(
        tx: Optional<RawTransaction>,
        spender: Script,
        amount: u128,
    ) -> Result<(), SSRIError>;
    fn allowance(owner: Script, spender: Script) -> Result<u128, SSRIError>;
    fn increase_allowance(
        tx: Optional<RawTransaction>,
        spender: Script,
        added_value: u128,
    ) -> Result<(), SSRIError>;
    fn decrease_allowance(
        tx: Optional<RawTransaction>,
        spender: Script,
        subtracted_value: u128,
    ) -> Result<(), SSRIError>;
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
    /* Note: Pausing/Unpause without lock would take effect on the global level */
    fn pause(tx: Optional<RawTransaction>, locks: Vec<Script>)
        -> Result<RawTransaction, SSRIError>;
    fn unpause(
        tx: Optional<RawTransaction>,
        locks: Vec<Script>,
    ) -> Result<RawTransaction, SSRIError>;
    fn is_paused(locks: Vec<Script>) -> Result<bool, SSRIError>;
    fn enumerate_paused() -> Result<Vec<Script>, SSRIError>;
}

#[derive(Serialize, Deserialize)]
pub struct UDTPausableData {
    pause_list: Vec<Byte32>,
    next_type_hash: Optional<Byte32>,
}

pub enum UDTPausableError {
    NoPausePermission,
    NoUnpausePermission,
    AbortedFromPause,
    IncompletePauseList,
}

pub enum UDTExtensionDataRegistryRecords {
    UDTPausableData = "UDTPausableData",
    UDTExtendedData = "UDTExtendedData",
}
