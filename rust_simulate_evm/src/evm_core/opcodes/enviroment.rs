use crate::evm_core::execute::Execute;
use crate::evm_core::utils::byte_operate::{bytes32_to_address, pad_left};
use crate::evm_core::utils::enviroment::get_balance;
use crate::evm_core::utils::error::RunnerError;

use ethers::types::U256;
use ethers::utils::keccak256;

/* -------------------------------------------------------------------------- */
/*                              Get env info from EVM                         */
/* -------------------------------------------------------------------------- */
/*     地址 余额信息      */
pub fn address(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = pad_left(&execute.address);
    let result = execute.stack.push(address);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn balance(execute: &mut Execute) -> Result<(), RunnerError> {
    let address: [u8; 32] = execute.stack.pop()?;
    let address: [u8; 20] = address[12..].try_into().unwrap();
    let balance = get_balance(address, execute)?;
    let result = execute.stack.push(pad_left(&balance));

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
/*     调用者信息      */
//交易发起者的地址tx.origin
pub fn origin(execute: &mut Execute) -> Result<(), RunnerError> {
    let origin = pad_left(&execute.origin);
    let result = execute.stack.push(origin);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn caller(execute: &mut Execute) -> Result<(), RunnerError> {
    let caller = pad_left(&execute.caller);
    let result = execute.stack.push(caller);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn callvalue(execute: &mut Execute) -> Result<(), RunnerError> {
    //调用发送的eth amount
    let result = execute.stack.push(execute.callvalue);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
/*     调用数据处理      */
pub fn calldataload(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = execute.stack.pop()?;
    let address = U256::from_big_endian(&address).as_usize();
    //读取并padding32字节的数据 压入堆栈
    let calldata = execute.calldata.read(address, 32)?;
    let calldata = calldata.as_slice().try_into().unwrap();
    let result = execute.stack.push(calldata);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn calldatasize(execute: &mut Execute) -> Result<(), RunnerError> {
    let size = execute.calldata.msize().to_be_bytes();
    // Convert the usize to bytes in little-endian order
    let calldatasize = pad_left(&size);
    let result = execute.stack.push(calldatasize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
//copy calldata to memory
pub fn calldatacopy(execute: &mut Execute) -> Result<(), RunnerError> {
    let dest_offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let _offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let _size = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let calldata = execute.calldata.read(_offset, _size)?;

    let result = execute.memory.write(dest_offset, calldata);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
/*     code operate      */
//return current contract codesize
pub fn codesize(execute: &mut Execute) -> Result<(), RunnerError> {
    let code = execute.state.get_code_at(execute.address);
    let codesize = if code.is_none() {
        [0u8; 32]
    } else {
        pad_left(&code.unwrap().len().to_be_bytes())
    };

    let result = execute.stack.push(codesize);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
//copy current contract code to memory
pub fn codecopy(execute: &mut Execute) -> Result<(), RunnerError> {
    let dest_offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let size = U256::from_big_endian(&execute.stack.pop()?).as_usize();

    let code = execute.state.get_code_at(execute.address);

    let code = if code.is_none() {
        vec![]
    } else {
        // Slice the code to the correct size
        let code = code.unwrap();
        let mut code_vec = code.to_vec();
        code_vec.resize(offset + size, 0);
        let code = code_vec.as_slice();
        code[offset..offset + size].to_vec()
    };
    execute.memory.write(dest_offset, code)?;

    execute.increase_pc(1)
}

//return special contract codesize
pub fn extcodesize(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = execute.stack.pop()?;
    let code = execute.state.get_code_at(bytes32_to_address(&address));
    let codesize = if code.is_none() {
        [0u8; 32]
    } else {
        pad_left(&code.unwrap().len().to_be_bytes())
    };

    let result = execute.stack.push(codesize);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
//copy special contract code to memory
pub fn extcodecopy(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = execute.stack.pop()?;
    let dest_offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let size = U256::from_big_endian(&execute.stack.pop()?).as_usize();

    let code = execute.state.get_code_at(bytes32_to_address(&address));
    let code = if code.is_none() {
        vec![]
    } else {
        // Slice the code to the correct size
        let code = code.unwrap();
        let mut code_vec = code.to_vec();
        code_vec.resize(32, 0);
        let code = code_vec.as_slice();
        code[offset..offset + size].to_vec()
    };
    execute.memory.write(dest_offset, code)?;

    execute.increase_pc(1)
}

//获取上一次调用返回的datasize并压入堆栈
pub fn returndatasize(execute: &mut Execute) -> Result<(), RunnerError> {
    let size = execute.returndata.msize().to_be_bytes();
    let returndatasize = pad_left(&size);

    let result = execute.stack.push(returndatasize);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}
//copy returndata
pub fn returndatacopy(execute: &mut Execute) -> Result<(), RunnerError> {
    let dest_offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let _offset = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let _size = U256::from_big_endian(&execute.stack.pop()?).as_usize();
    let returndata = execute.returndata.read(_offset, _size)?;
    let result = execute.memory.write(dest_offset, returndata);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

/*     环境变量      */
pub fn extcodehash(execute: &mut Execute) -> Result<(), RunnerError> {
    let address = execute.stack.pop()?;

    Ok(
        if let Some(code) = execute.state.get_code_at(bytes32_to_address(&address)) {
            let codehash = keccak256(code);
            let result = execute.stack.push(codehash);
            if result.is_err() {
                return Err(result.unwrap_err());
            }

            execute.increase_pc(1);
        },
    )
}

pub fn blockhash(execute: &mut Execute) -> Result<(), RunnerError> {
    let block: U256 = U256::from_big_endian(&execute.stack.pop()?);
    let mut bytes = [0; 32];
    block.to_big_endian(&mut bytes);
    let blockhash = keccak256(bytes);

    let result = execute.stack.push(blockhash);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

//block.coinbase 当前区块的矿工地址
pub fn coinbase(execute: &mut Execute) -> Result<(), RunnerError> {
    let coinbase = match &execute.evm_context {
        None => pad_left(&[0xc0u8; 20]),
        Some(evm_context) => {
            if let Some(coinbase) = evm_context.coinbase {
                pad_left(&coinbase)
            } else {
                pad_left(&[0xc0u8; 20])
            }
        }
    };

    let result = execute.stack.push(coinbase);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn timestamp(execute: &mut Execute) -> Result<(), RunnerError> {
    let timestamp_secs = match &execute.evm_context {
        None => pad_left(&[0x00; 20]),
        Some(evm_context) => {
            if let Some(timestamp_secs) = evm_context.timestamp {
                timestamp_secs
            } else {
                pad_left(&[0x00; 20])
            }
        }
    };
    let result = execute.stack.push(timestamp_secs);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn number(execute: &mut Execute) -> Result<(), RunnerError> {
    let number = match &execute.evm_context {
        None => pad_left(&[0xff; 4]),
        Some(evm_context) => {
            if let Some(number) = evm_context.block_number {
                number
            } else {
                pad_left(&[0xff; 4])
            }
        }
    };
    let result = execute.stack.push(number);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn difficulty(execute: &mut Execute) -> Result<(), RunnerError> {
    //硬编码
    let difficulty = pad_left(&[0x45; 8]);
    let result = execute.stack.push(difficulty);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn gaslimit(execute: &mut Execute) -> Result<(), RunnerError> {
    let gaslimit = match &execute.evm_context {
        // gas：30_000_000 30M
        None => pad_left(&[0x01, 0xC9, 0xC3, 0x80]),
        Some(evm_context) => {
            if let Some(gaslimit) = evm_context.gas_limit {
                gaslimit
            } else {
                pad_left(&[0x01, 0xC9, 0xC3, 0x80])
            }
        }
    };

    let result = execute.stack.push(gaslimit);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

pub fn chainid(execute: &mut Execute) -> Result<(), RunnerError> {
    let chainid = pad_left(&[0x01]);
    let result = execute.stack.push(chainid);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

//current contract balance
pub fn selfbalance(execute: &mut Execute) -> Result<(), RunnerError> {
    let balance = get_balance(execute.address, execute)?;

    let result = execute.stack.push(balance);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}

//basefee = 10
pub fn basefee(execute: &mut Execute) -> Result<(), RunnerError> {
    let basefee = match &execute.evm_context {
        None => pad_left(&[0x0a]),
        Some(evm_context) => {
            if let Some(basefee) = evm_context.basefee {
                basefee
            } else {
                pad_left(&[0x0a])
            }
        }
    };

    let result = execute.stack.push(basefee);
    if result.is_err() {
        return Err(result.unwrap_err());
    }

    execute.increase_pc(1)
}