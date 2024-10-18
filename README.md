# ckb_ssri_sdk

## Goals

1. Easier definitions and implementations of SSRI namespacing;
2. Unified entry function;

## Execution Environment Levels

- Code: If only the code hash is provided, the execution environment level is Code;
- Script: If the code hash and args are both provided, the execution environment level is Script;
- Cell: If the CellInput / Outpoint is provided, the execution environment level is Cell;
- Transaction: If the Transaction Hash is provided, the execution environment level is Transaction; This is also the level of execution environment for regular CKB-VM when submitting transaction;

## Usage

1. SSRI-Compliant Smart Contract Code would have an identifier Type that also implements TypeID mechanism.
2. SSRI-Compliant Smart Contract would use the unified entry function to run the script and call the exposed methods by specifying the path at `argv[0]` and the arguments at `argv[1..]`.
    - The default namespace is `SSRI` which consists of:
        - `SSRI.version() -> u8`
        - `SSRI.get_methods(offset: u64, limit: u64) -> Vec<Bytes8>`
        - `SSRI.has_methods(methods: Vec<Bytes8>) -> Vec<bool>`
    - By using the `#[ssri_module]` macro and `#[ssri_method]` attribute, the developer can define their own namespace and methods and automatically expose them in the methods of the default namespace `SSRI`.
3. By implementing traits from `ckb_ssri_sdk::public_module_traits` in the SSRI-Compliant Smart Contract, infrastructures would be able to provide richer information off-chain as well for all kinds of purposes based on the SSRI protocol.


```sh
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "run_script",
    "params": ["<TxHash of the target Cell>", <Index>, [<Bytes of methods path>, <...argv>]]
}'
```

## Defining a SSRI Module

### #[ssri_module]

- base: Share the same namespace
- version: implement a method to get the version of the SSRI module.

### #[ssri_method]

- By default, all of the following flags are set to false or empty;
- 'level={ExecutionEnvironmentLevel}': This method can only be run when the execution environment level is above or equal to the specified level;
- 'transaction=true': Will return a transaction object of molecule `struct` that can be sent to directly to RPC. If transaction is set to true, the required level is automatically set to Chain;
- 'internal=true': This method is not exposed through SSRI, but it's a dependency for other methods;

## Defining a SSRI Public Module Trait
