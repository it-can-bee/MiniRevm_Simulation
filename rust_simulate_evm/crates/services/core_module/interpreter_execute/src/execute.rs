use colored::Colorize;
use lib_utils::error::RunnerError;
use context::evm_context::EvmContext;

use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::EvmState;
use crate::enviroment::*;
use crate::assembly::*;

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
        Ok(())
    }

    /*==============解析器==================*/
    pub fn interpret(
        &mut self,
        bytecode: Vec<u8>,
        initial_interpretation: bool,
    ) -> Result<(), RunnerError> {

        Ok(())
    }
}