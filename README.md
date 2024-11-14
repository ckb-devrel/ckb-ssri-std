# ckb_ssri_sdk
>
> [[EN/CN] Script-Sourced Rich Information - 来源于 Script 的富信息](https://talk.nervos.org/t/en-cn-script-sourced-rich-information-script/8256): General introduction to SSRI.
>
> [`ssri-test`](https://github.com/Hanssen0/ssri-test): First prototype of SSRI-Compliant contract.
>
> [`ssri-server`](https://github.com/ckb-devrel/ssri-server): Server for calling SSRI methods.
>
> [`ckb_ssri_cli`](https://github.com/Alive24/ckb_ssri_cli): Command Line Interface for general users, moderators, and devs to interact with SSRI-Compliant Contracts deployed on CKB Network.
## About
`ckb_ssri_sdk` is a toolkit to help developers build SSRI-Compliant smart contracts on CKB by providing:
- Public Module Traits which would receive first party infrastructure support across the ecosystem, such as CKB Explorer, JoyID wallet, etc.
- Useful utility functions and macros to simplify the experience of building SSRI-Compliant contract
- Production level example contract `pausable-udt` for reference.

## Quick Note on SSRI

SSRI stands for `Script Sourced Rich Information`; it is a protocol for strong bindings of relevant information and conventions to the Script itself on CKB. For more information, please read [[EN/CN] Script-Sourced Rich Information - 来源于 Script 的富信息](https://talk.nervos.org/t/en-cn-script-sourced-rich-information-script/8256)>.

Such bindings would take place in a progressive pattern:
1. On the level of validating transactions, by specifically using Rust Traits, we recognize the purpose (or more specifically, the `Intent` of running the script) (e.g., `minting UDT`, `transferring`) and build relevant validation logics within the scope of the corresponding method.
2. On the level of reading and organizing contract code, by selectively implementing methods of public module traits (e.g. `UDT`, `UDTExtended`, `UDTPausable`) in combinations, generic users and devs would be able to quickly understand and organize functionalities of contracts as well as the relevant adaptations / integrations in dApps , especially in use cases involving multiple distinct contracts (and very likely from different projects) within same transactions.
3. On the level of dApp integration and interactions with `ckb_ssri_cli`, SSRI-Compliant contracts provide predictable interfaces for information query (e.g. generic metadata source for explorer, CCC integration for pubic trait methods such as UDT), transaction generation/completion, and output data calculations which reduces engineering workload significantly by sharing code effectively.
  
## Goals of `ckb_ssri_sdk`

- Easier and intuitive implementations and built-in integration support (e.g. `CCC` and `ckb_ssri_cli`) of SSRI public traits.
- Easier and intuitive definitions of customized SSRI traits.
- [ ] TODO: Unified entry function;
- Scrip-Sourced code sharing for on-chain verification, off-chain query/integration, and off-chain transaction generations/completions.

## Usage

1. [ ] TODO: SSRI-Compliant Smart Contract Code would have an identifier Type that also implements TypeID mechanism. Please use [`ckb-cinnabar`](https://github.com/ashuralyk/ckb-cinnabar?tab=readme-ov-file#deployment-module) for easier deployment and migration.
2. SSRI-Compliant Smart Contract would use the unified entry function to run the script and call the exposed methods by specifying the path at `argv[0]` and the arguments at `argv[1..]`.
    - The default namespace is `SSRI` which consists of:
        - `SSRI.version() -> u8`
        - `SSRI.get_methods(offset: u64, limit: u64) -> Vec<Bytes8>`
        - `SSRI.has_methods(methods: Vec<Bytes8>) -> Vec<bool>`
    - [ ] TODO: By using the `#[ssri_module]` macro and `#[ssri_method]` attribute, methods can be automatically exposed in the namespace defined by trait name.
3. By implementing traits from `ckb_ssri_sdk::public_module_traits` in the SSRI-Compliant Smart Contract, infrastructures would be able to provide richer information off-chain as well for all kinds of purposes based on the SSRI protocol.

## Example Contract

[`pausable-udt`](https://github.com/Alive24/ckb_ssri_sdk/tree/main/contracts/pausable-udt) is a real production level contract (instead of a pseudo-project) that exemplifies the usage of SSRI.

## [ ] TODO: Defining a SSRI Module with `proc-macros` and Implement SSRI Traits

### #[ssri_module]

- version: implement a method to get the version of the SSRI module.

### #[ssri_method]

- By default, all of the following flags are set to false or empty;
- 'level={ExecutionEnvironmentLevel}': This method can only be run when the execution environment level is above or equal to the specified level;
- 'transaction=true': Will return a transaction object of molecule `struct` that can be sent to directly to RPC. If transaction is set to true, the required level is automatically set to Chain;
- 'internal=true': This method is not exposed through SSRI, but it's a dependency for other methods;

## Defining a SSRI Public Module Trait

## Deployment and Migration

- Deploy and upgrade with [ckb-cinnabar](https://github.com/ashuralyk/ckb-cinnabar?tab=readme-ov-file#deployment-module) for easier deployment and migration with Type ID.

```shell
ckb-cinnabar deploy --contract-name pausable-udt --tag transaction.v241112 --payer-address ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqtxe0gs9yvwrsc40znvdc6sg4fehd2mttsngg4t4 --type-id 


ckb-cinnabar migrate --contract-name pausable-udt --from-tag v241030.1 --to-tag v241030.2
```

## Interacting with SSRI-Compliant Contracts

With SSRI-server and SSRI-VM, users, devs, and admins would have the ability to obtain Script-Source Rich Information directly from the script by calling SSRI methods via SSRI-server (which can be both run locally and for public usage).

The returned information can be just data for display, or a transaction object that can be sent to a CKB RPC, without the need of extra implementations on either dApps or backend applications.

```sh
echo '{
    "id": 2,
    "jsonrpc": "2.0",
    "method": "run_script",
    "params": ["<TxHash of the target Cell>", <Index>, [<Bytes of methods path>, <...argv>]]
}'
```

### `ckb_ssri_cli`

## Testing

Due to the limitations of `ckb_testtools`, it is recommended to test the same SSRI-Compliant Contract on two level:

- On-chain Verification: Test with `ckb_testtools`
- Off-chain Query/Integration, Transaction Generations/Completions: Test with `ckb_ssri_cli` against the latest deployment.

## Key Concepts

### Execution Environment Levels

- Code: If only the code hash is provided, the execution environment level is Code;
- Script: If the code hash and args are both provided, the execution environment level is Script;
- Cell: If the CellInput / Outpoint is provided, the execution environment level is Cell;
- Transaction: If the Transaction Hash is provided, the execution environment level is Transaction; This is also the level of execution environment for regular CKB-VM when submitting transaction;
