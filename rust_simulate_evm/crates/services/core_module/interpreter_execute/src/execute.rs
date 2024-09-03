use lib_utils::error::RunnerError;
use crate::stack::Stack;
use crate::memory::Memory;
use crate::storage::EvmState;

#[derive(Debug)]
pub struct Execute {
    //execute
    pub pc: usize,
    pub bytecode: Vec<u8>,

    // Environment
    pub gas: u64,
    pub address: [u8; 20],

    //data
    pub stack:Stack,
    pub memory: Memory,
    pub returndata: Memory,
    pub state: EvmState,
}

impl Execute {
    pub fn new() -> Self {
        Self {
            pc: 0,
            bytecode: Vec::new(),
            gas: 0,
            address: [0u8; 20],
            stack: Stack::new(),
            memory: Memory::new(None),
            returndata: Memory::new(None),
            state: EvmState::new(None),
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
}