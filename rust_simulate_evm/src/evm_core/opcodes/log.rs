use crate::evm_core::execute::Execute;
use crate::evm_core::log::Log;

use crate::evm_core::utils::error::RunnerError;
use super::super::gas::constant::{VERYLOW_LOG0, VERYLOW_LOG1, VERYLOW_LOG2, VERYLOW_LOG3, VERYLOW_LOG4};
use ethers::types::U256;

pub fn log0(execute: &mut Execute) -> Result<(), RunnerError> {
    // Check if static mode is enabled
    if execute.state.static_mode {
        return Err(RunnerError::StaticCallStateChanged);
    }
    if execute.gas < VERYLOW_LOG0 {
        return Err(RunnerError::OutOfGas)
    }

    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size = U256::from_big_endian(&execute.stack.pop()?);

    let log_data =  execute.memory.read(offset.as_usize(), size.as_usize())?;

    let log = Log {
        address: execute.address,
        topics: vec![],
        data: log_data.clone(),
    };

    execute.state.logs.push(log);

    // Increment PC
    execute.increase_pc(1)
}

pub fn log1(execute: &mut Execute) -> Result<(), RunnerError> {
    // Check if static mode is enabled
    if execute.state.static_mode {
        return Err(RunnerError::StaticCallStateChanged);
    }
    if execute.gas < VERYLOW_LOG1 {
        return Err(RunnerError::OutOfGas)
    }

    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size: U256 = U256::from_big_endian(&execute.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let log_data = execute.memory.read(offset.as_usize(), size.as_usize())?;

    let log = Log {
        address: execute.address,
        topics: vec![topic1],
        data: log_data.clone(),
    };

    execute.state.logs.push(log);

    // Increment PC
    execute.increase_pc(1)
}

pub fn log2(execute: &mut Execute) -> Result<(), RunnerError> {
    // Check if static mode is enabled
    if execute.state.static_mode {
        return Err(RunnerError::StaticCallStateChanged);
    }
    if execute.gas < VERYLOW_LOG2 {
        return Err(RunnerError::OutOfGas)
    }

    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size: U256 = U256::from_big_endian(&execute.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let log_data = execute.memory.read(offset.as_usize(), size.as_usize())?;

    let log = Log {
        address: execute.address,
        topics: vec![topic1, topic2],
        data: log_data.clone(),
    };

    execute.state.logs.push(log);

    // Increment PC
    execute.increase_pc(1)
}

pub fn log3(execute: &mut Execute) -> Result<(), RunnerError> {
    // Check if static mode is enabled
    if execute.state.static_mode {
        return Err(RunnerError::StaticCallStateChanged);
    }
    if execute.gas < VERYLOW_LOG3 {
        return Err(RunnerError::OutOfGas)
    }

    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size: U256 = U256::from_big_endian(&execute.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let log_data = execute.memory.read(offset.as_usize(), size.as_usize())? ;

    let log = Log {
        address: execute.address,
        topics: vec![topic1, topic2, topic3],
        data: log_data.clone(),
    };

    execute.state.logs.push(log);

    // Increment PC
    execute.increase_pc(1)
}

pub fn log4(execute: &mut Execute) -> Result<(), RunnerError> {
    // Check if static mode is enabled
    if execute.state.static_mode {
        return Err(RunnerError::StaticCallStateChanged);
    }
    if execute.gas < VERYLOW_LOG4 {
        return Err(RunnerError::OutOfGas)
    }

    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size: U256 = U256::from_big_endian(&execute.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let raw_topic4: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut topic4 = [0u8; 32];
    raw_topic4.to_big_endian(&mut topic4);

    let log_data = execute.memory.read(offset.as_usize(), size.as_usize())?;

    let log = Log {
        address: execute.address,
        topics: vec![topic1, topic2, topic3, topic4],
        data: log_data.clone(),
    };

    execute.state.logs.push(log);

    // Increment PC
    execute.increase_pc(1)
}

