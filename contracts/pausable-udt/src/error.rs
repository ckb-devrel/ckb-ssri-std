use core::str::Utf8Error;

use ckb_std::error::SysError;
use ckb_ssri_sdk::SSRIError;
use ckb_ssri_sdk::public_module_traits::udt::{UDTMetadataError, UDTExtendedError, UDTPausableError, UDTError};

/// Error
#[repr(i8)]
#[derive(Debug)]
pub enum Error {
    // * CKB Error
    IndexOutOfBound = 1,
    ItemMissing = 2,
    LengthNotEnough,
    Encoding,
    SpawnExceededMaxContentLength,
    SpawnWrongMemoryLimit,
    SpawnExceededMaxPeakMemory,

    // * Rust Error
    Utf8Error,

    // * SSRI Error
    SSRIMethodsNotFound,
    SSRIMethodsArgsInvalid,
    SSRIMethodsNotImplemented,
    SSRIMethodRequireHigherLevel,


    // * Serde Molecule Error
    SerdeMoleculeErrorWithMessage, 
    /// Contains a general error message as a string.
    /// Occurs when the data length is incorrect while parsing a number or molecule header.
    MismatchedLength,
    /// Occurs when the data length is insufficient while parsing a number or molecule header.
    SerdeMoleculeLengthNotEnough,
    /// Indicates that the method or type is not implemented. Not all types in Rust can be serialized.
    Unimplemented,
    /// Occurs when assembling a molecule fixvec, and the size of each element is inconsistent.
    AssembleFixvec,
    /// Occurs when the header or size is incorrect while parsing a molecule fixvec.
    InvalidFixvec,
    /// Occurs when the field count is mismatched while parsing a molecule table.
    MismatchedTableFieldCount,
    /// Occurs when an overflow happens while parsing a molecule header.
    Overflow,
    /// Indicates an error encountered while parsing a molecule array.
    InvalidArray,
    /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
    InvalidStructField,
    /// Indicates that a map should have exactly two fields: a key and a value.
    InvalidMap,
    /// Indicates that the table header is invalid or malformed.
    InvalidTable,
    /// Indicates that the table length is invalid or malformed.
    InvalidTableLength,
    /// Indicates that the table header is invalid or malformed.
    InvalidTableHeader,
    /// Indicates that the field count in serialization is mismatched.
    InvalidTableCount,
    /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
    MixTableAndStruct,
    InvalidChar,

    // * UDT Error
    InsufficientBalance,

    // * UDT Metadata Error
    NameUndefined,
    SymbolUndefined,
    DecimalsUndefined,
    TotalSupplyUndefined,
    CapUndefined,
    ExtensionDataNotFound,


    // * UDT Extended Error
    NoTransferPermission,
    NoMintPermission,
    NoBurnPermission,
    NoApprovePermission,
    NoIncreaseAllowancePermission,
    NoDecreaseAllowancePermission,

    // * UDT Pausable Error
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

impl From<Utf8Error> for Error {
    fn from(_err: Utf8Error) -> Self {
        Self::Utf8Error
    }
}

impl From<serde_molecule::Error> for Error {
    fn from(err: serde_molecule::Error) -> Self {
        use serde_molecule::Error::*;
        match err {
                /// Contains a general error message as a string.
            Message(string) => Self::SerdeMoleculeErrorWithMessage,
            /// Occurs when the data length is incorrect while parsing a number or molecule header.
            MismatchedLength => Self::MismatchedLength,
            /// Occurs when the data length is insufficient while parsing a number or molecule header.
            LengthNotEnough => Self::SerdeMoleculeLengthNotEnough,
            /// Indicates that the method or type is not implemented. Not all types in Rust can be serialized.
            Unimplemented => Self::Unimplemented,
            /// Occurs when assembling a molecule fixvec, and the size of each element is inconsistent.
            AssembleFixvec => Self::AssembleFixvec,
            /// Occurs when the header or size is incorrect while parsing a molecule fixvec.
            InvalidFixvec => Self::InvalidFixvec,
            /// Occurs when the field count is mismatched while parsing a molecule table.
            MismatchedTableFieldCount => Self::MismatchedTableFieldCount,
            /// Occurs when an overflow happens while parsing a molecule header.
            Overflow => Self::Overflow,
            /// Indicates an error encountered while parsing a molecule array.
            InvalidArray => Self::InvalidArray,
            /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
            InvalidStructField => Self::InvalidStructField,
            /// Indicates that a map should have exactly two fields: a key and a value.
            InvalidMap => Self::InvalidMap,
            /// Indicates that the table header is invalid or malformed.
            InvalidTable => Self::InvalidTable,
            /// Indicates that the table length is invalid or malformed.
            InvalidTableLength => Self::InvalidTableLength,
            /// Indicates that the table header is invalid or malformed.
            InvalidTableHeader => Self::InvalidTableHeader,
            /// Indicates that the field count in serialization is mismatched.
            InvalidTableCount => Self::InvalidTableCount,
            /// Indicates that non-fixed size fields are not allowed in a molecule struct, e.g., `Option`, `Vec`, `DynVec`, `enum`.
            MixTableAndStruct => Self::MixTableAndStruct,
            InvalidChar => Self::InvalidChar,
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

