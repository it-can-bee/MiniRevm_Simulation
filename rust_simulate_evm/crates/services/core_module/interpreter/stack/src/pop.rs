use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;

pub fn pop(executor: &mut Execute) -> Result<(), RunnerError> {
    match executor.stack.pop() {
        Ok(_value) => {
            executor.increase_pc(1)
        },
        Err(e) => Err(e)
    }
}