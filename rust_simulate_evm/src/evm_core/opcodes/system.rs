use ethers::types::{U256};
use ethers::utils::keccak256;
use revm_primitives::Address;
use crate::evm_core::utils::error::RunnerError;
use crate::evm_core::execute::Execute;

use super::super::utils::byte_operate::{pad_left, bytes32_to_address, u64_to_u256_array};
use super::super::utils::enviroment::
{
    delete_account,
    get_balance,
    get_nonce,
    init_account,
};

pub fn invalid(execute: &mut Execute) -> Result<(), RunnerError> {
    Err(RunnerError::InvalidOpcode(execute.bytecode[execute.pc]))
}

pub fn create(execute: &mut Execute) -> Result<(), RunnerError> {
    let value = execute.stack.pop()?;
    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size = U256::from_big_endian(&execute.stack.pop()?);
    //从指定的内存位置读取init_code
    let init_code = execute.memory.read(offset.as_usize(), size.as_usize())?;

    // 使用地址的nonce计算新合约地址
    let nonce = get_nonce(
        execute.address,
        execute,
    )?;
    let nonce = U256::from_big_endian(&nonce).0[0];
    let caller = &execute.caller;
    //caller, nonce 用于派生生成合约地址
    let create_address = Address::from_slice(caller).create(nonce);
    // 创建账户并存储init_code到init_account
    init_account(*create_address.0, execute)?;
    execute
        .state
        .put_code_at(*create_address.0, init_code)?;
    //构造函数
    let call_result = execute.call(*create_address.0, value, Vec::new(), execute.gas, false);

    if call_result.is_err() {
        execute
            .stack
            .push(pad_left(&[0x00]))?;
    } else {
        execute
            .stack
            .push(pad_left(&*create_address.0))?;
    }
    //存储合约代码
    let returndata = execute.returndata.heap.clone();
    execute
        .state
        .put_code_at(*create_address.0, returndata.clone())?;
    execute
        .state
        .transfer(execute.caller, *create_address.0, value)?;

    execute.increase_pc(1)
}
//可以根据salt和init code hash推导contract address
pub fn create2(execute: &mut Execute) -> Result<(), RunnerError> {
    let value = execute.stack.pop()?;
    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size = U256::from_big_endian(&execute.stack.pop()?);

    let salt = execute.stack.pop()?;
    let init_code = execute.memory.read(offset.as_usize(), size.as_usize())?;
    let init_code_hash = keccak256(init_code.clone());
    let caller = &execute.caller;

    // caller, init_code_hash, salt => precompute Address
    let create_address = Address::from_slice(caller).create2(salt, init_code_hash);
    //初始化合约账户状态
    init_account(*create_address.0, execute)?;
    execute.state.put_code_at(*create_address.0, init_code)?;

    let call_result = execute.call(*create_address.0, value, Vec::new(), execute.gas, false);

    if call_result.is_err() {
        execute
            .stack
            .push(pad_left(&[0x00]))?;
    } else {
        execute
            .stack
            .push(pad_left(&(*create_address.0)))?;
    }

    let returndata = execute.returndata.heap.clone();
    execute
        .state
        .put_code_at(*create_address.0, returndata)?;
    execute
        .state
        .transfer(execute.caller, *create_address.0, value)?;

    execute.increase_pc(1)
}

pub fn selfdestruct(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = execute.stack.pop()?;
    let contract_balance = get_balance(execute.address, execute)?;

    execute.state.transfer(
        execute.address,
        bytes32_to_address(&address),
        contract_balance,
    )?;
    delete_account(execute.address, execute)?;

    execute.increase_pc(1)
}


/*  合约交互   */
pub fn call(execute: &mut Execute, bypass_static: bool) -> Result<(), RunnerError> {
    if execute.state.static_mode && !bypass_static {
        return Err(RunnerError::StaticCallStateChanged);
    }

    let gas = execute.stack.pop()?;
    let to = execute.stack.pop()?;

    let value = if bypass_static {
        [0u8; 32]
    } else {
        execute.stack.pop()?
    };

    let calldata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let calldata_size = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_size = U256::from_big_endian(&execute.stack.pop()?);

    let calldata = execute.memory.read
                                                (
                                                    calldata_offset.as_usize(),
                                                    calldata_size.as_usize()
                                                )?;

    let call_result = execute.call(
        bytes32_to_address(&to),
        value,
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        false,
    );

    if call_result.is_err() {
        execute.stack.push(pad_left(&[0x00]))?;
    } else {
        execute.stack.push(pad_left(&[0x01]))?;
    }
    //将调用后的返回数据写回内存
    let mut return_data: Vec<u8> = execute.returndata.heap.clone();

    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    return_data = return_data[0..returndata_size.as_usize()].to_vec();
    execute
        .memory
        .write(returndata_offset.as_usize(), return_data)?
    ;
    // 对于CALLCODE，value为[0u8; 32]，因为CALLCODE不会转移value
    if !value.eq(&[0u8; 32]) {
        execute
            .state
            .transfer(execute.address, bytes32_to_address(&to), value)?;
    }

    execute.increase_pc(1)
}

//CALL修改的是被调用者的storage，而CALLCODE修改的是调用者的storage
pub fn callcode(execute: &mut Execute,
                bypass_static: bool
) -> Result<(), RunnerError>
{
    if execute.state.static_mode && !bypass_static {
        return Err(RunnerError::StaticCallStateChanged);
    }

    // 弹出堆栈中的值
    let gas = execute.stack.pop()?;
    let to = execute.stack.pop()?;
    let value = execute.stack.pop()?;
    let calldata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let calldata_size = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_size = U256::from_big_endian(&execute.stack.pop()?);

    // 从内存中读取 calldata
    let calldata = execute.memory.read(
        calldata_offset.as_usize(),
        calldata_size.as_usize(),
    )?;

    // 调用被调用者的代码，但保留调用者的存储
    let call_result = execute._call_inner(
        bytes32_to_address(&to),
        value,
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        true,
    );

    if call_result.is_err() {
        execute.stack.push(pad_left(&[0x00]))?;
    } else {
        execute.stack.push(pad_left(&[0x01]))?;
    }

    let mut return_data: Vec<u8> = execute.returndata.heap.clone();
    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    // 处理返回数据并写入内存
    return_data = return_data[0..returndata_size.as_usize()].to_vec();
    execute.memory.write(returndata_offset.as_usize(), return_data)?;
    // callcode不转移value
    execute.increase_pc(1)
}

//callcode已废弃
pub fn _callcode(_: &mut Execute) -> Result<(), RunnerError> {
    Err(RunnerError::NotImplemented(0xF2))
}

pub fn delegatecall(execute: &mut Execute) -> Result<(), RunnerError> {
    let gas = execute.stack.pop()?;
    let to = execute.stack.pop()?;

    let calldata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let calldata_size = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_offset = U256::from_big_endian(&execute.stack.pop()?);
    let returndata_size = U256::from_big_endian(&execute.stack.pop()?);

    let calldata = execute
                                .memory
                                .read(calldata_offset.as_usize(), calldata_size.as_usize())?;

    let call_result = execute.call(
        bytes32_to_address(&to),
        [0u8; 32],
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        true,
    );

    if call_result.is_err() {
        execute.stack.push(pad_left(&[0x00]))?;
    } else {
        execute.stack.push(pad_left(&[0x01]))?;
    }

    let mut return_data: Vec<u8> = execute.returndata.heap.clone();

    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    return_data = return_data[0..returndata_size.as_usize()].to_vec();
    execute
        .memory
        .write(returndata_offset.as_usize(), return_data)?;

    execute.increase_pc(1)
}

//call but static，only for reading
pub fn staticcall(execute: &mut Execute) -> Result<(), RunnerError> {
    execute.state.static_mode = true;
    let result = call(execute, true);
    execute.state.static_mode = false;
    result
}

//从当前合约返回数据 终止执行
pub fn return_(execute: &mut Execute) -> Result<(), RunnerError> {
    let offset = U256::from_big_endian(&execute.stack.pop()?);
    let size = U256::from_big_endian(&execute.stack.pop()?);

    let returndata = unsafe { execute.memory.read(offset.as_usize(), size.as_usize())? };
    execute.returndata.heap = returndata;

    // 当前合约执行完毕
    execute.set_pc(execute.bytecode.len());

    Ok(())
}
