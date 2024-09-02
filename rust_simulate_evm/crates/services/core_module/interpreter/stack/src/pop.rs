use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use gas::constant::VERYLOW_2;

pub fn pop(execute: &mut Execute) -> Result<(), RunnerError> {
    if execute.gas < VERYLOW_2 {
        return Err(RunnerError::OutOfGas)
    }
    match execute.stack.pop() {
        Ok(_value) => {
            execute.increase_pc(1)
        },
        Err(e) => Err(e)
    }
}