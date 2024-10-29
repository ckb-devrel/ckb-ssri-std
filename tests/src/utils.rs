use ckb_hash::blake2b_256;
use ckb_std::{
    ckb_types::{bytes::Bytes, packed::*, prelude::*},
    high_level::encode_hex,
};
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use reqwest::Client;

use crate::Loader;

pub fn method_path(name: impl AsRef<[u8]>) -> u64 {
    u64::from_le_bytes(blake2b_256(name)[0..8].try_into().unwrap())
}

pub fn method_path_hex(name: impl AsRef<[u8]>) -> String {
    let method_path = method_path(name);
    let method_path_in_bytes = method_path.to_le_bytes();
    let method_path_hex = format!(
        "0x{:?}",
        encode_hex(&method_path_in_bytes).into_string().unwrap()
    )
    .replace("\"", "");
    method_path_hex
}

pub async fn get_ssri_response(payload: serde_json::Value) -> serde_json::Value {
    let url = "http://localhost:9090";

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .expect("Request failed");

    // Assert that the request was successful (status 200)
    assert!(
        response.status().is_success(),
        "Response was not successful"
    );

    let response_json: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    response_json
}

pub struct PausableUDTTestContext {
    pub context: Context,
    pub pausable_udt_out_point: OutPoint,
    pub always_success_dep: CellDep,
    pub pausable_udt_dep: CellDep,
    pub pausable_udt_type_script: Script,
    pub admin_lock_script: Script,
    pub normal_user_a_lock_script: Script,
    pub normal_user_b_lock_script: Script,
    pub paused_user_lock_script: Script,
}

pub fn build_test_context() -> PausableUDTTestContext {
    let admin_args = String::from("00018fd14cc327648651dc0ac81ec6dd63a9ab376e61");
    let normal_user_a_args = String::from("00018fd14cc327648651dc0ac81ec6dd63a9ab376e62");
    let normal_user_b_args = String::from("00018fd14cc327648651dc0ac81ec6dd63a9ab376e63");
    let paused_user_args = String::from("00018fd14cc327648651dc0ac81ec6dd63a9ab376e64");

    let mut context = Context::default();
    let loader = Loader::default();
    let pausable_udt_bin = loader.load_binary("pausable-udt");
    let pausable_udt_out_point = context.deploy_cell(pausable_udt_bin);
    let pausable_udt_dep = CellDep::new_builder()
        .out_point(pausable_udt_out_point.clone())
        .build();
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    let always_success_dep = CellDep::new_builder()
        .out_point(always_success_out_point.clone())
        .build();
    let always_success_script = context
        .build_script(&always_success_out_point, Bytes::default())
        .expect("script");

    let fake_lock_outpoint = OutPoint::new_builder()
        .tx_hash(Byte32::zero())
        .index(0u32.pack())
        .build();

    let fake_lock_dep = CellDep::new_builder()
        .out_point(fake_lock_outpoint.clone())
        .build();

    let fake_lock_cell_output = CellOutput::new_builder()
        .capacity(1000u64.pack())
        .lock(always_success_script.clone())
        .build();

    context.create_cell_with_out_point(
        fake_lock_outpoint.clone(),
        fake_lock_cell_output,
        ALWAYS_SUCCESS.clone(),
    );

    let fake_lock_script = context
        .build_script(&fake_lock_outpoint, Bytes::from(admin_args.clone()))
        .expect("script");

    println!(
        "fake_lock_script.calc_script_hash(): {}",
        fake_lock_script.calc_script_hash()
    );

    let admin_lock_script = context
        .build_script(&fake_lock_outpoint.clone(), Bytes::from(admin_args))
        .expect("script");
    let normal_user_a_lock_script = context
        .build_script(&fake_lock_outpoint.clone(), Bytes::from(normal_user_a_args))
        .expect("script");
    let normal_user_b_lock_script = context
        .build_script(&fake_lock_outpoint.clone(), Bytes::from(normal_user_b_args))
        .expect("script");
    let paused_user_lock_script = context
        .build_script(&fake_lock_outpoint.clone(), Bytes::from(paused_user_args))
        .expect("script");

    let pausable_udt_type_script = context
        .build_script(
            &pausable_udt_out_point,
            admin_lock_script.calc_script_hash().as_bytes(),
        )
        .expect("script");

    let paused_user_lock_script_hash_byte32 = paused_user_lock_script.calc_script_hash();
    let paused_user_lock_script_hash_hex =
        encode_hex(paused_user_lock_script_hash_byte32.as_slice());

    println!(
        "paused_user_lock_script_hash_hex: {}",
        paused_user_lock_script_hash_hex.into_string().unwrap()
    );

    PausableUDTTestContext {
        context,
        pausable_udt_out_point,
        always_success_dep,
        pausable_udt_dep,
        pausable_udt_type_script,
        admin_lock_script,
        normal_user_a_lock_script,
        normal_user_b_lock_script,
        paused_user_lock_script,
    }
}
