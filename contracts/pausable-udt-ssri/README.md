# pausable-udt-ssri

This is a SSRI-compliant smart contract that implements a pausable UDT (User-Defined Token) with the SSRI protocol.

> Quick Note On SSRI:
>

## Implementations

### UDT

### UDTMetadata

### UDTPausable

```rust
#[ssri_method(level="transaction", transaction=true)]
fn pause(lock: Option<Script>) -> Result<(), SSRIError>;
```

- When executed in SSRI-VM, this method will return a transaction object that can be sent to RPC directly to pause the lock script.
- If no lock has been passed, the method will perform a global pause.

```rust
#[ssri_method(level="transaction", transaction=true)]
fn unpause(lock: Option<Script>) -> Result<(), SSRIError>;
```

- Symmetric to `pause`.

```rust
#[ssri_method(level="chain")]
fn is_paused(locks: Vec<Script>) -> Result<Vec<bool>, SSRIError>;
```

- When executed in SSRI-VM, this method will return if the lock scripts have been paused from minting and transferring directly.
- If the length of the vector is 0, the method will check if the token is paused globally.

### UDTMetadataData

### UDTPausableData

A `Vec<UDTPausableData>` would be stored at UDTMetadataData::extension_data_registry::pausable_data. Each UDTPausableData would store paused lock script hashes in `Vec<Bytes32>` and also an optional pointer to the type hash of another cell (with Unique Cell Implementation) that contains extra lock script hashes in the case of prolonged length of entries. In this way, you can either store all the paused lock script hashes in the metadata cell or extend in a linked style if the list is long; in the latter case, if you only need to confirm that a specific lock script has been paused, you don't need to provide all the chains for CellDep but only one that can be traced back to the only metadata cell (you still need to enumerate them all if you need to prove the otherwise case).

## How to Use

### General Users

- Through direct calling or integrations with the SSRI-server in dApps, users should be able to see and validate if specific lock scripts have been paused from minting and transferring and the specific transactions that have paused them.
- Users with paused lock scripts would get `UDTPausableError::AbortedFromPause` when trying to mint or transfer tokens.

### Admins and Devs

### Further Extensions

- When executed in a transaction, this method will validate if the lock script has been paused from minting and transferring in the transaction; when
