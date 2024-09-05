use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use gas::constant::VERYLOW;

use crate::core
use crate::core_module::utils::errors::ExecutionError;


pub fn dup1(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(1);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup2(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(2);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup3(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(3);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup4(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(4);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup5(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(5);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup6(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(6);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup7(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(7);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup8(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(8);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup9(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(9);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup10(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(10);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup11(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(11);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup12(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(12);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup13(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(13);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup14(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(14);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup15(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(15);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}

pub fn dup16(execute: &mut Execute) ->  Result<(), RunnerError> {
    if execute.gas < VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    let result = execute.stack.dup(16);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increase_pc(1)
}