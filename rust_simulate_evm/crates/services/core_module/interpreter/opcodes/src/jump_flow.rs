use colored::*;
use ethers::types::U256;

use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use lib_core::{byte_operate::pad_left, debug};

pub fn stop(execute: &mut Execute) -> Result<(), RunnerError>  {
    //将pc指向末尾
    execute.set_pc(execute.bytecode.len());
    Ok(())
}

pub fn revert(execute: &mut Execute) -> Result<(), RunnerError>  {
    //从内存中读取数据
    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size = U256::from_big_endian(&execute.stack.pop()?);
    let revert_data = execute.memory.read(offset.as_usize(), size.as_usize());
    //读取到returndata.heap
    execute.returndata.heap = revert_data.as_ref().unwrap().to_owned();

    //回滚状态返回的信息
    let err;
    let hex;
    if revert_data.is_ok() && revert_data.as_ref().unwrap().len() > 0 {
        hex = debug::vec_to_hex_string(
            revert_data.as_ref().unwrap().as_slice().try_into().unwrap(),
        );
        println!("Revert data: {}", hex.red());
        err = RunnerError::Revert(revert_data.unwrap());
    } else {
        hex = debug::to_hex_string([0u8; 32]);
        println!("Revert without data: {}", hex.red());
        err = RunnerError::RevertWithoutData;
    }

    Err(err)
}

pub fn jump(execute: &mut Execute) -> Result<(), RunnerError>  {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&execute.stack.pop()?);
    jump_address.to_big_endian(&mut bytes);

    if jump_address.as_usize() > execute.bytecode.len() {
        return Err(RunnerError::OutOfBoundsByteCode);
    }
    // Check destination is JUMPDEST 0x5b
    if execute.bytecode[jump_address.as_usize()] != 0x5b {
        return Err(RunnerError::InvalidJumpDestination);
    }
    execute.set_pc(jump_address.as_usize());

    Ok(())
}
//次栈顶非0跳转到临时栈顶位置
pub fn jumpi(execute: &mut Execute) -> Result<(), RunnerError>  {
    let mut bytes = [0u8; 32];
    let jump_address = U256::from_big_endian(&execute.stack.pop()?);
    jump_address.to_big_endian(&mut bytes);
    let destination = U256::from_big_endian(&execute.stack.pop()?);

    // Check bounds
    if jump_address.as_usize() > execute.bytecode.len() {
        return Err(RunnerError::OutOfBoundsByteCode);
    }

    // Check condition
    if !destination.is_zero() {
        execute.set_pc(jump_address.as_usize());
    } else {
        let _ = execute.increase_pc(1);
    }
    // Check destination
    if execute.bytecode[jump_address.as_usize()] != 0x5b {
        return Err(RunnerError::InvalidJumpDestination);
    }

    Ok(())
}

/* -------------------------------------------------------------------------- */
/*                              Push currency to stack                        */
/* -------------------------------------------------------------------------- */
pub fn pc(execute: &mut Execute) -> Result<(), RunnerError>  {
    let pc = execute.get_pc().to_be_bytes();
    let pc = pad_left(&pc.to_vec());

    execute.stack.push(pc)?;
    execute.increase_pc(1)
}

pub fn gas(execute: &mut Execute) -> Result<(), RunnerError>  {
    let gas = execute.gas.to_be_bytes();
    let gas = pad_left(&gas.to_vec());

    execute.stack.push(gas)?;
    execute.increase_pc(1)
}

//mark
pub fn jumpdest(execute: &mut Execute) -> Result<(), RunnerError>  {
    execute.increase_pc(1)
}