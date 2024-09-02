use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use gas::constant::{VERYLOW, VERYLOW_2};

use ethers::types::U256;

pub fn mload(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    // 从栈中弹出32字节作为memory加载的偏移量
    let offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let word = execute.memory.mload(offset)?;
    let result = execute.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn mstore(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let offset = U256::from_big_endian(&execute.stack.pop()?);
    //弹出offset 然后弹出data
    let data = execute.stack.pop()?;
    let result = execute.memory.mstore(offset.as_usize(), data);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn mstore8(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let data = execute.stack.pop()?;
    let byte_value = data[31];

    let mut store_data = [0u8; 32];
    // 将目标字节放在正确的位置
    let byte_index = offset % 32;
    store_data[byte_index] = byte_value;
    // 计算对齐后的存储地址
    let aligned_offset = offset - byte_index;

    let result = execute.memory.mstore(aligned_offset, store_data);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}


pub fn msize(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW_2 {
        return Err(RunnerError::OutOfGas)
    }
    let mut bytes_msize = [0u8; 32];
    U256::from(execute.memory.msize()).to_big_endian(&mut bytes_msize);

    let result = execute.stack.push(bytes_msize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

// EIP-5656: mcopy - Memory copying instruction
pub fn mcopy(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    // 从栈中弹出目标偏移地址destOffset 源偏移地址offset
    let dest_offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let size = U256::from_big_endian(&execute.stack.pop()?).as_usize();

    let result = execute.memory.mcopy(dest_offset, offset, size);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}