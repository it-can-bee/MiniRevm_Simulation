use crate::evm_core::utils::error::RunnerError;
use crate::evm_core::execute::Execute;
use super::super::gas::constant::{VERYLOW_SSTORE, VERYLOW_SLOAD};

pub fn sload(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW_SLOAD {
        return Err(RunnerError::OutOfGas)
    }
    //从弹出的指定地址读取值
    let slot_address = execute.stack.pop()?;
    let word = execute.state.sload(execute.address, slot_address)?;

    let result = execute.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn sstore(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW_SSTORE {
        return Err(RunnerError::OutOfGas)
    }
    //将值存储到指定的slot里边
    let slot_address = execute.stack.pop()?;
    let word = execute.stack.pop()?;

    let result = execute.state.sstore(execute.address, slot_address, word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increase_pc(1)
}

