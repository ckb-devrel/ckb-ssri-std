// Rust serde_molecule implementation of following schemas:
// https://github.com/nervosnetwork/ckb/blob/develop/util/gen-types/schemas/blockchain.mol

use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_molecule::{dynvec_serde, struct_serde};


#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Debug)]
pub struct Script {
    pub code_hash: [u8; 32],
    pub hash_type: u8,
    pub args: Vec<u8>,
}