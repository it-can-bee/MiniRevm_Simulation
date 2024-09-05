use crate::evm_core::utils::byte_operate::{_hex_string_to_bytes};
use std::collections::HashMap;
use ethers::types::U256;

use super::memory::Memory;
use super::opcodes;
use super::stack::Stack;
use super::storage::{AccountState, EvmState};
use super::utils;
use crate::evm_core::utils::enviroment::{increment_nonce, init_account};
use super::utils::error::RunnerError;

// Colored output
use colored::*;
// use crate::evm_core::context::account_state_ex_context::AccountStateEx;
use crate::evm_core::context::evm_context::EvmContext;
use crate::evm_core::utils::assembly::get_op_code;

use crate::debug;



#[derive(Debug)]
pub struct Execute {
    //execute
    pub pc: usize,
    pub bytecode: Vec<u8>,
    pub call_depth: u32,

    // Environment
    pub gas: u64,
    pub address: [u8; 20],
    pub origin: [u8; 20],
    pub caller: [u8; 20],
    pub callvalue: [u8; 32],

    //data
    pub stack:Stack,
    pub memory: Memory,
    pub returndata: Memory,
    pub calldata: Memory,
    pub state: EvmState,

    // EVM env
    pub evm_context: Option<EvmContext>,

    // EVM op_count
    pub op_count: u128,

}

impl Execute {
    pub fn new(
        caller: [u8; 20],
        origin: Option<[u8; 20]>,
        address: Option<[u8; 20]>,
        callvalue: Option<[u8; 32]>,
        calldata: Option<Vec<u8>>,
        state: Option<EvmState>,
        evm_context: Option<EvmContext>,
    ) -> Self {
        Self {
            //execute
            pc: 0,
            bytecode: Vec::new(),
            // Environment
            gas: 30_000_000, //[0x01, 0xC9, 0xC3, 0x80]
            address: if address.is_some() {
                address.unwrap()
            } else {
                [0x5fu8; 20]
            },
            origin: if origin.is_some() {
                origin.unwrap()
            } else {
                caller
            },
            caller,
            callvalue: if callvalue.is_some() {
                callvalue.unwrap()
            } else {
                [0u8; 32]
            },
            //data
            stack: Stack::new(),
            memory: Memory::new(None),
            returndata: Memory::new(None),
            calldata: Memory::new(calldata),
            state: if state.is_some() {
                state.unwrap()
            } else {
                EvmState::new(None)
            },
            // EVM env
            evm_context: evm_context,
            call_depth: 0,
            op_count: 0, // EVM op_count
        }
    }

    pub fn increase_pc(&mut self, size:usize) -> Result<(), RunnerError> {
        self.pc += size;
        Ok(())
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    //上下文切换
    pub fn call(
        &mut self,
        to: [u8; 20],
        value: [u8; 32],
        calldata: Vec<u8>,
        _gas: u64,
        delegate: bool,
    ) -> Result<(), RunnerError> {
        let mut error: Option<RunnerError> = None;

        // Store the initial runner state
        let initial_caller = self.caller.clone();
        let initial_callvalue = self.callvalue.clone();
        let initial_address = self.address.clone();

        let initial_calldata = self.calldata.clone();
        let initial_returndata = self.returndata.clone();

        let initial_memory = self.memory.clone();
        let initial_stack = self.stack.clone();
        let initial_pc = self.pc.clone();
        let initial_bytecode = self.bytecode.clone();

        // 状态更新和环境设置
        if !delegate {
            self.caller = self.address.clone();
            self.callvalue = value;
            self.address = to;
        }

        self.call_depth += 1;
        self.calldata = Memory::new(Some(calldata));
        self.returndata = Memory::new(None);

        self.memory = Memory::new(None);
        self.stack = Stack::new();
        self.pc = 0;

        // 重新加载和执行字节码
        let code = self.state.get_code_at(to);
        if code.is_some() {
            let interpret_result = self.interpret(code.unwrap().to_owned(), false);
            if interpret_result.is_err() {
                error = Some(interpret_result.unwrap_err());
            }
        }
        let return_data = self.returndata.heap.clone();

        if !delegate {
            // call状态恢复
            self.caller = initial_caller;
            self.callvalue = initial_callvalue;
            self.address = initial_address;
        }
        //执行完毕后会恢复调用前的状态
        self.calldata = initial_calldata;
        self.returndata = initial_returndata;

        self.memory = initial_memory;
        self.stack = initial_stack;
        self.pc = initial_pc;
        self.bytecode = initial_bytecode;
        self.call_depth -= 1;
        self.returndata.heap = return_data;

        increment_nonce(self.address, self)?;

        if let Some(err) = error {
            return Err(err);
        }

        Ok(())
    }
    //callcode 已废弃
    pub fn _call_inner(
        &mut self,
        to: [u8; 20],
        value: [u8; 32],
        calldata: Vec<u8>,
        _gas: u64,
        is_callcode: bool
    ) -> Result<(), RunnerError> {
        let mut error: Option<RunnerError> = None;

        // Store the initial runner state
        let initial_caller = self.caller.clone();
        let initial_callvalue = self.callvalue.clone();
        let initial_address = self.address.clone();
        let initial_calldata = self.calldata.clone();
        let initial_returndata = self.returndata.clone();
        let initial_memory = self.memory.clone();
        let initial_stack = self.stack.clone();
        let initial_pc = self.pc.clone();
        let initial_bytecode = self.bytecode.clone();

        // 状态更新和环境设置
        if !is_callcode {
            // CALL操作，更新caller和address
            self.caller = self.address.clone();
            self.callvalue = value;
            self.address = to;
        } else {
            // CALLCODE操作，不更改caller和address，只更改callvalue
            self.callvalue = value;
        }

        self.call_depth += 1;
        self.calldata = Memory::new(Some(calldata));
        self.returndata = Memory::new(None);

        self.memory = Memory::new(None);
        self.stack = Stack::new();
        self.pc = 0;

        // 重新加载和执行字节码
        let code_address = if is_callcode {
            // CALLCODE，使用caller的存储
            self.address
        } else {
            // CALL操作，使用被调用者to的存储
            to
        };

        let code = self.state.get_code_at(code_address);

        if let Some(code) = code {
            let interpret_result = self.interpret(code.to_owned(), false);
            if interpret_result.is_err() {
                error = Some(interpret_result.unwrap_err());
            }
        }

        // 获取返回数据
        let return_data = self.returndata.heap.clone();

        // 恢复调用前的环境状态
        if !is_callcode {
            self.caller = initial_caller;
            self.callvalue = initial_callvalue;
            self.address = initial_address;
        }

        self.calldata = initial_calldata;
        self.returndata = initial_returndata;
        self.memory = initial_memory;
        self.stack = initial_stack;
        self.pc = initial_pc;
        self.bytecode = initial_bytecode;
        self.call_depth -= 1;

        // 将返回数据写回初始状态
        self.returndata.heap = return_data;

        // 增加调用者的nonce
        increment_nonce(self.address, self)?;

        if let Some(err) = error {
            return Err(err);
        }

        Ok(())
    }

    /*==============解析器==================*/
    pub fn interpret(
        &mut self,
        bytecode: Vec<u8>,
        initial_interpretation: bool,
    ) -> Result<(), RunnerError> {
        self.bytecode = bytecode;

        if initial_interpretation {
            // Set the runner address code
            let put_code_result = self.state.put_code_at(self.address, self.bytecode.clone());
            if put_code_result.is_err() {
                return Err(put_code_result.unwrap_err());
            }
        }

        /* -------------------------------------------------------------------------- */
        /*                             Interpret bytecode                             */
        /* -------------------------------------------------------------------------- */
        let mut error: Option<RunnerError> = None;
        if self.bytecode.is_empty() {
            println!("{}: {}", "ERROR: ".red(), "EmptyByteCode");
            return Err(RunnerError::EmptyByteCode);
        }

        let mut op_list = Vec::new();
        // Interpret the bytecode
        while self.pc < self.bytecode.len() {
            let mut op_count = self.op_count;
            let mut flag = [0u8; 30];
            for i in 1..30 {
                if self.call_depth.eq(&i) && flag[i as usize] == 0 {
                    flag[i as usize] = 1;
                    op_count += i as u128;
                }
            }

            // Interpret an opcode
            op_list.push(get_op_code(self.bytecode[self.pc]));

            let my_opcode = get_op_code(self.bytecode[self.pc]).to_string();
            /*=======================逐条处理操作码 (Opcode Execution)=========================*/
            //负责根据提供的操作码调用相应的处理函数
            //每种操作码对应一个具体的函数，这些函数定义在op_codes模块
            let result = self.interpret_op_code(self.bytecode[self.pc]);
            if result.is_err() {
                error = Some(result.unwrap_err());
                break;
            }
            self.op_count += 1;
        }
        /* -------------------------------------------------------------------------- */
        /*                            Print execution error                           */
        /* -------------------------------------------------------------------------- */

        if error.is_some() {
            println!(
                "{} {}\n  {}: 0x{:X}\n  {}: 0x{:X}\n  {}\n op_count: {}",
                "ERROR:".red(),
                "Runtime error".red(),
                "PC".yellow(),
                self.pc,
                "OpCode".yellow(),
                self.bytecode[self.pc],
                //error.as_ref().unwrap().to_string().red()
                format!("{:?}", error.as_ref().unwrap()),
                self.op_count
            );

            return Err(error.unwrap());
        }

        Ok(())
    }

    /* 轮询执行每个opcode */
    pub fn interpret_op_code(&mut self, opcode: u8) -> Result<(), RunnerError> {
        match opcode {
           _ => println!("Hello my cycle reference"),
        }
        Ok(())
    }

    /*==============调试器==================*/
    fn debug_stack(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n\n{}", border_line.clone().green());
        println!(
            "{} {:<101} {}",
            "║".green(),
            "Final stack".yellow(),
            "║".green()
        );

        println!("{}", footer_line.clone().green());
        let mut reversed_stack = self.stack.stack.clone();
        reversed_stack.reverse();

        for (_, element) in reversed_stack.iter().enumerate() {
            let hex: String = debug::to_hex_string(*element);
            println!("{}", hex);
        }
    }

    /// Print a debug message that display the final memory.
    fn debug_memory(&self) {
        let border_line =
            "\n╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝\n";

        println!("\n{}", border_line.clone().blue());
        println!(
            "{} {:<101} {}",
            "║".blue(),
            "Final memory heap".yellow(),
            "║".blue()
        );
        println!("{}", footer_line.blue());

        for chunk in self.memory.heap.chunks(32) {
            let padded_chunk: Vec<u8>;

            if chunk.len() < 32 {
                // If the chunk size is less than 32, create a new vector with enough zeros to reach a total size of 32
                padded_chunk = [chunk.to_vec(), vec![0u8; 32 - chunk.len()]].concat();
            } else {
                // If the chunk size is exactly 32, use it as is
                padded_chunk = chunk.to_vec();
            }

            let hex: String =
                debug::to_hex_string(padded_chunk.as_slice().try_into().unwrap());
            println!("{}", hex);
        }

        if self.memory.heap.is_empty() {
            println!("🚧 {} 🚧", "Empty memory".red());
        }

        println!();
    }

    fn debug_storage(&mut self) {
        self.state.debug_state();
    }
}