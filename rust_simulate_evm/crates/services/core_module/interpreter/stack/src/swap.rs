use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;

pub fn swap1(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(1);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap2(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(2);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap3(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(3);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap4(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(4);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap5(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(5);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap6(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(6);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap7(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(7);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    execute.increment_pc(1)
}

pub fn swap8(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(8);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap9(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(9);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap10(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(10);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap11(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(11);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap12(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(12);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap13(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(13);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap14(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(14);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap15(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(15);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}

pub fn swap16(execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.stack.swap(16);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    execute.increment_pc(1)
}
