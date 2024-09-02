use colored::*;
/*数据格式*/
use ethers::types::U256;
use std::str::FromStr;
/*地址哈希 交易哈希 区块哈希*/
use primitive_types::{H160, H256};

//为长度不固定的字节数组填充到32字节
pub fn pad_left(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}
//数据掩码 填充255
pub fn pad_left_one(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [255u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

pub fn _pad_right(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[..bytes.len()].copy_from_slice(bytes);
    padded
}

/* -------------------------------------------------------------------------- */
/*                               Data Translate                             */
/* -------------------------------------------------------------------------- */
pub fn bytes32_to_address(bytes: &[u8; 32]) -> [u8; 20] {
    let mut address = [0u8; 20];
    address.copy_from_slice(&bytes[12..]);
    address
}

//还原数据 移除填充的0
pub fn strip_zero_padding(arr: &[u8; 32]) -> &[u8] {
    //remote left padding
    let start = arr
                        .iter()
                        //find the start of the nozero
                        .position(|&x| x != 0)
                        .unwrap_or(0);
    //remote right padding
    let end = arr
                        .iter()
                        //find the last of the nozero
                        .rposition(|&x| x != 0)
                        .unwrap_or(0) + 1;
    &arr[start..end]
}


pub fn u64_to_u256_array(n: u64) -> [u8; 32] {
    let uint256 = U256::from(n);
    let mut bytes = [0u8; 32];
    uint256.to_big_endian(&mut bytes);
    bytes
}

pub fn u64_x4_array_to_u8_x32_array(arr: U256) -> [u8; 32] {
    let mut arr_u8 = [0u8; 32];
    arr.to_big_endian(&mut arr_u8);
    arr_u8
}

pub fn _hex_string_to_bytes(hex: &str) -> Vec<u8> {
    let mut after_hex = hex;
    if hex.starts_with("0x") {
        after_hex = &hex[2..];
    }
    match hex::decode(after_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("Error: {}", e.to_string().red());
        }
    }
}

/* -------------------------------------------------------------------------- */
/*                               Math operations                              */
/* -------------------------------------------------------------------------- */
pub fn _add(arr: [u8; 32], number: u64) -> [u8; 32] {
    // Convert the [u8; 32] into U256
    let num = U256::from_big_endian(&arr);

    // Add
    let num = num + U256::from(number);

    // Convert back to [u8; 32]
    let mut result = [0u8; 32];
    num.to_big_endian(&mut result);

    result
}

pub fn to_h160(str_address: &'static str) -> H160 {
    H160::from_str(str_address).unwrap()
}

pub fn to_h256(str_address: &'static str) -> H256 {
    H256::from_str(str_address).unwrap()
}
