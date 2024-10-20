use ckb_std::error::SysError;
use ckb_ssri_sdk::SSRIError;
use ckb_ssri_sdk::public_module_traits::udt::{UDTMetadataError, UDTExtendedError, UDTPausableError, UDTError};

/// Error
#[repr(i8)]
#[derive(Debug)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,
    SpawnExceededMaxContentLength,
    SpawnWrongMemoryLimit,
    SpawnExceededMaxPeakMemory,

    SSRIMethodsNotFound,
    SSRIMethodsArgsInvalid,
    SSRIMethodsNotImplemented,
    SSRIMethodRequireHigherLevel,

    InsufficientBalance,

    NameUndefined,
    SymbolUndefined,
    DecimalsUndefined,
    TotalSupplyUndefined,
    CapUndefined,
    ExtensionDataNotFound,

    NoTransferPermission,
    NoMintPermission,
    NoBurnPermission,
    NoApprovePermission,
    NoIncreaseAllowancePermission,
    NoDecreaseAllowancePermission,

    NoPausePermission,
    NoUnpausePermission,
    AbortedFromPause,
    IncompletePauseList,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            SpawnExceededMaxContentLength => Self::SpawnExceededMaxContentLength,
            SpawnWrongMemoryLimit => Self::SpawnWrongMemoryLimit,
            SpawnExceededMaxPeakMemory => Self::SpawnExceededMaxPeakMemory,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}

impl From<SSRIError> for Error {
    fn from(err: SSRIError) -> Self {
        match err {
            SSRIError::SSRIMethodsNotFound => Self::SSRIMethodsArgsInvalid,
            SSRIError::SSRIMethodsArgsInvalid => Self::SSRIMethodsNotImplemented,
            SSRIError::SSRIMethodsNotImplemented => Self::SSRIMethodsNotImplemented,
            SSRIError::SSRIMethodRequireHigherLevel => Self::SSRIMethodRequireHigherLevel,
        }
    }
}

impl From<UDTError> for Error {
    fn from(err: UDTError) -> Self {
        match err {
            UDTError::InsufficientBalance => Self::InsufficientBalance,
        }
    }
}

impl From<UDTMetadataError> for Error {
    fn from(err: UDTMetadataError) -> Self {
        match err {
            UDTMetadataError::NameUndefined => Self::NameUndefined,
            UDTMetadataError::SymbolUndefined => Self::SymbolUndefined,
            UDTMetadataError::DecimalsUndefined => Self::DecimalsUndefined,
            UDTMetadataError::TotalSupplyUndefined => Self::TotalSupplyUndefined,
            UDTMetadataError::CapUndefined => Self::CapUndefined,
            UDTMetadataError::ExtensionDataNotFound => Self::ExtensionDataNotFound,
        }
    }
}

impl From<UDTPausableError> for Error {
    fn from(err: UDTPausableError) -> Self {
        match err {
            UDTPausableError::NoPausePermission => Self::NoPausePermission,
            UDTPausableError::NoUnpausePermission => Self::NoUnpausePermission,
            UDTPausableError::AbortedFromPause => Self::AbortedFromPause,
            UDTPausableError::IncompletePauseList => Self::IncompletePauseList,
        }
    }
}

