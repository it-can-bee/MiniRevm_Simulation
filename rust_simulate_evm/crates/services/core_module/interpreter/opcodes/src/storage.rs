use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use gas::constant::{VERYLOW_SSTORE, VERYLOW_SLOAD};

pub fn sload(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas > VERYLOW_SLOAD {
        return Err(RunnerError::OutOfGas)
    }
    let address = execute.stack.pop()?;
    let word = execute.state.sload(execute.address, address)?;

    let result = execute.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn sstore(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas > VERYLOW_SSTORE {
        return Err(RunnerError::OutOfGas)
    }
    let address = execute.stack.pop()?;
    let word = execute.stack.pop()?;

    let result = execute.state.sstore(execute.address, address, word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increase_pc(1)
}

