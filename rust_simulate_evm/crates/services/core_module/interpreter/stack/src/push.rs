use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;
use gas::constant::{VERYLOW, VERYLOW_2};


//数据右对齐 计算偏移量
fn prepare_data(data: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let start = 32 - data.len();
    padded[start..].copy_from_slice(data);
    padded
}

//从字节码中提取数据，并将其正确地放入栈中 数据32字节空间内右对齐
pub fn push(execute: &mut Execute, data_len: usize) -> Result<(), RunnerError> {
    if execute.gas > VERYLOW {
        return Err(RunnerError::OutOfGas)
    }
    if execute.pc + 1 + data_len > execute.bytecode.len() {
        return Err(RunnerError::OutOfBoundsByteCode);
    }

    let data = &execute.bytecode[execute.pc + 1..execute.pc + 1 + data_len];
    let padded = prepare_data(data);
    let result = execute.stack.push(padded);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1 + data_len)

}