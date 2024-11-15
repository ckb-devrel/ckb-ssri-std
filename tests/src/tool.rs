use ckb_std::high_level::{decode_hex, encode_hex};

#[test]
fn decode_hex_tool() {
    let hex = "";
    let bytes = decode_hex(hex);
    println!("Decoded Bytes: {:?}", bytes);
}


#[test]
fn encode_hex_tool() {
    let data = "";
    let hex = encode_hex(data.as_bytes());
    println!("Encoded Hex: {:?}", hex);
}

