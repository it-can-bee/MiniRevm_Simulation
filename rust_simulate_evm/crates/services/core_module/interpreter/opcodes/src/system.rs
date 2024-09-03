use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;


pub fn invalid(execute: &mut Execute) -> Result<(), RunnerError> {
    Err(RunnerError::InvalidOpcode(execute.bytecode[execute.pc]))
}

pub fn create(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}

pub fn create2(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}

pub fn selfdestruct(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}


/*  合约交互   */
pub fn call(execute: &mut Execute, bypass_static: bool) -> Result<(), RunnerError> {
    Ok(())
}

pub fn callcode(_: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}

pub fn delegatecall(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}

pub fn staticcall(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}

pub fn return_(execute: &mut Execute) -> Result<(), RunnerError> {
    Ok(())
}
