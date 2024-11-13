# Pausable UDT

## Usage

- Deploy and upgrade with [ckb-cinnabar](https://github.com/ashuralyk/ckb-cinnabar?tab=readme-ov-file#deployment-module) for easier deployment and migration with Type ID.

```shell
ckb-cinnabar deploy --contract-name pausable-udt --tag transaction.v241112 --payer-address ckt1qzda0cr08m85hc8jlnfp3zer7xulejywt49kt2rr0vthywaa50xwsqtxe0gs9yvwrsc40znvdc6sg4fehd2mttsngg4t4 --type-id 


ckb-cinnabar migrate --contract-name pausable-udt --from-tag v241030.1 --to-tag v241030.2
```

- Interact with [ckb_ssri_cli](https://github.com/Alive24/ckb_ssri_cli)
    - `ckb_ssri_cli udt:balance`: Balance checking
    - `ckb_ssri_cli udt:transfer`: Transfer UDT
    - `ckb_ssri_cli udt:extended:mint`: Mint UDT
    - `ckb_ssri_cli udt:pausable:is-paused`: Check if the lock hash is paused
    - `ckb_ssri_cli udt:pausable:enumerate-paused`: Enumerate paused lock hashes.
