# Pausable UDT

## Usage

- Deploy with [ckb-cinnabar](https://github.com/ashuralyk/ckb-cinnabar?tab=readme-ov-file#deployment-module) for easier deployment and migration with Type ID.
- Interact with [ckb_ssri_cli](https://github.com/Alive24/ckb_ssri_cli)
    - `ckb_ssri_cli udt:balance`: Balance checking
    - `ckb_ssri_cli udt:transfer`: Transfer UDT
    - `ckb_ssri_cli udt:extended:mint`: Mint UDT
    - `ckb_ssri_cli udt:pausable:is-paused`: Check if the lock hash is paused
    - `ckb_ssri_cli udt:pausable:enumerate-paused`: Enumerate paused lock hashes.
