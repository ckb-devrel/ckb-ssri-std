use std::ffi::CStr;
use std::ffi::CString;

use ckb_std::ckb_types::packed::Bytes as PackedBytes;
use ckb_std::ckb_types::packed::BytesBuilder as PackedBytesBuilder;
use ckb_std::ckb_types::packed::BytesVec;
use ckb_std::ckb_types::packed::Script;
use ckb_std::ckb_types::packed::ScriptOpt;
use ckb_std::ckb_types::packed::ScriptOptBuilder;
use ckb_std::ckb_types::packed::Uint64;
use ckb_std::{
    ckb_types::packed::{CellOutputBuilder, Uint128},
    high_level::{decode_hex, encode_hex},
};
use ckb_testtool::ckb_types::prelude::{Builder, Entity, Pack, Unpack};

// #[test]
// fn decode_hex_tool() {
//     let hex = "";
//     let bytes = decode_hex(hex);
//     println!("Decoded Bytes: {:?}", bytes);
// }

// #[test]
// fn encode_hex_tool() {
//     let data = "";
//     let hex = encode_hex(data.as_bytes());
//     println!("Encoded Hex: {:?}", hex);
// }

// #[test]
// fn generic_test() {
//     let to_lock_bytes_vec = BytesVec::from_compatible_slice(&[
//         85, 0, 0, 0, 8, 0, 0, 0, 73, 0, 0, 0, 73, 0, 0, 0, 16, 0, 0, 0, 48, 0, 0, 0, 49, 0, 0, 0,
//         155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142,
//         93, 75, 101, 168, 99, 123, 23, 114, 59, 189, 163, 204, 232, 1, 20, 0, 0, 0, 11, 254, 38,
//         33, 79, 170, 88, 32, 191, 73, 222, 62, 81, 32, 242, 83, 245, 218, 96, 247,
//     ])
//     .unwrap();
//     println!("to_lock_bytes_vec: {:?}", to_lock_bytes_vec);
//     println!("First item: {:?}", to_lock_bytes_vec.get(0).unwrap());
//     // let new_script = Script::new_unchecked(to_lock_bytes_vec.get(0).unwrap().unpack());

//     let to_lock_vec: Vec<Script> = to_lock_bytes_vec.into_iter().map(|bytes| Script::new_unchecked(bytes.unpack())).collect();
//     println!("to_lock_vec: {:?}", to_lock_vec);
// }
