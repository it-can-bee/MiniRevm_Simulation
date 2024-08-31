use lib_utils::error::RunnerError;
use interpreter_execute::execute::Execute;


pub const VERYLOW: u64 = 3;

//数据右对齐 计算偏移量
fn prepare_data(data: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    let start = 32 - data.len();
    padded[start..].copy_from_slice(data);
    padded
}

//从字节码中提取数据，并将其正确地放入栈中 数据32字节空间内右对齐
pub fn push(executor: &mut Execute, data_len: usize) -> Result<(), RunnerError> {
    if (executor.gas > VERYLOW) {
        return Err(RunnerError::OutOfGas)
    }
    if executor.pc + 1 + data_len > executor.bytecode.len() {
        return Err(RunnerError::OutOfBoundsByteCode);
    }

    let data = &executor.bytecode[executor.pc + 1..executor.pc + 1 + data_len];
    let padded = prepare_data(data);
    let result = executor.stack.push(padded);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    executor.increase_pc(1 + data_len)

}