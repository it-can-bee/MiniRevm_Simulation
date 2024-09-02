use lib_core::byte_operate::pad_left;
use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;

// Primitive types
use ethers::types::{I256, U256};

pub fn iszero(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;

    let a = U256::from_big_endian(&pop1);

    let bool = a.is_zero();

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn eq(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;
    let pop2 = execute.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.eq(&b);

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn lt(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;
    let pop2 = execute.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.lt(&b);

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn gt(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;
    let pop2 = execute.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let bool = a.gt(&b);

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn slt(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;
    let pop2 = execute.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let bool = a.lt(&b);

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn sgt(execute: &mut Execute) -> Result<(), RunnerError> {
    let pop1 = execute.stack.pop()?;
    let pop2 = execute.stack.pop()?;

    let a = I256::from_raw(U256::from_big_endian(&pop1));
    let b = I256::from_raw(U256::from_big_endian(&pop2));

    let bool = a.gt(&b);

    let result_bytes = pad_left(&[if bool { 1u8 } else { 0u8 }; 1]);

    let result = execute.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}


