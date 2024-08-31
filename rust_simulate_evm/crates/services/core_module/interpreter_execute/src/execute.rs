use lib_utils::error::RunnerError;
//use crate::modules::stack::Stack;
use crate::stack::Stack;

pub struct Execute {
    //execute
    pub pc: usize,
    pub bytecode: Vec<u8>,

    // Environment
    pub gas: u64,

    //data
    pub stack:Stack,
}

impl Execute {
    pub fn new() -> Self {
        Self {
            pc: 0,
            bytecode: Vec::new(),
            gas: 0,
            stack: Stack::new(),
        }
    }

    pub fn increase_pc(&mut self, size:usize) -> Result<(), RunnerError> {
        self.pc += size;
        Ok(())
    }
}