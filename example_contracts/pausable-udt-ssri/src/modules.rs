use ckb_ssri_sdk::public_module_traits::{UDTExtended, UDTMetadata, UDTPausable, UDT};

#[ssri_module]
pub struct UDTSSRI;

impl UDT for UDTSSRI {
    #[ssri_method(level="cell")]
    fn balance() -> Result<u128, SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
    #[ssri_method(level="transaction", transaction=true)]
    fn transfer(from: Script, to: Script, amount: u128) -> Result<(), SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
}

#[ssri_module(base=UDT)]
impl UDTMetadata for UDTSSRI {
    /** Note: If the UDT is issued with a generic UDT Type and defines it's metadata in CellDep, it would require Chain level; if it is only compliant to the SSRI trait UDT and is able to return name/symbol/decimals within the script, and it would require only code/script level. */
    #[ssri_method(level="transaction")]
    fn name() -> Result<Bytes, SSRIError> {
        todo!()
    }
    #[ssri_method(level="transaction")]
    fn symbol() -> Result<Bytes, SSRIError> {
        todo!()
    }
    #[ssri_method(level="transaction")]
    /* Note: By default, decimals are 8 when decimals() are not implemented */
    fn decimals() -> Result<u8, SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
}

#[ssri_module(base=UDT)]
impl UDTExtended for UDTSSRI {
    #[ssri_method(level="transaction", transaction=true)]
    fn mint(lock: Script, amount: u128) -> Result<(), SSRIError> {
        todo!()
    }
    
    fn approve(
        tx: Optional<RawTransaction>,
        spender: Script,
        amount: u128,
    ) -> Result<(), SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
    
    fn allowance(owner: Script, spender: Script) -> Result<u128, SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
    
    fn increase_allowance(
        tx: Optional<RawTransaction>,
        spender: Script,
        added_value: u128,
    ) -> Result<(), SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }
    
    fn decrease_allowance(
        tx: Optional<RawTransaction>,
        spender: Script,
        subtracted_value: u128,
    ) -> Result<(), SSRIError> {
        Err(SSRIError::SSRIMethodsNotImplemented)
    }


}

#[ssri_module(base=UDT)]
impl UDTPausable for UDTSSRI {
    #[ssri_method(level="transaction", transaction=true)]
    fn pause(tx: Optional<RawTransaction>, locks: Vec<Script>)
        -> Result<RawTransaction, SSRIError> {
        todo!()
    }

    #[ssri_method(level="transaction", transaction=true)]
    fn unpause(
        tx: Optional<RawTransaction>,
        locks: Vec<Script>,
    ) -> Result<RawTransaction, SSRIError> {
        todo!()
    }

    #[ssri_method(level="transaction", transaction=true)]
    fn is_paused(locks: Vec<Script>) -> Result<bool, SSRIError> {
        todo!()
    }

    #[ssri_method(level="transaction", transaction=true)]
    fn enumerate_paused() -> Result<Vec<Script>, SSRIError> {
        todo!()
    }
}