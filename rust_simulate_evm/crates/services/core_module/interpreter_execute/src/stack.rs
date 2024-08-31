use lib_utils::error::RunnerError;
pub const STACK_LIMIT:usize = 1024;
#[derive(Debug)]
pub struct Stack {
    /// The stack itself
    pub stack: Vec<[u8; 32]>,
}


impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn pop(&mut self) -> Result<[u8; 32], RunnerError> {
        if self.stack.is_empty() {
            return Err(RunnerError::StackTooSmall);
        }

        Ok(self.stack.pop().unwrap())
    }

    pub fn push(&mut self, data_len: [u8; 32]) -> Result<(), RunnerError> {
        // Check if the stack has exceeded the limit
        if self.stack.capacity() > STACK_LIMIT {
            return Err(RunnerError::StackOverflow);
        }
        if self.stack.len() >= STACK_LIMIT {
            return Err(RunnerError::StackTooDeep);
        }

        Ok( self.stack.push(data_len) )
    }

    // pub fn swap(&mut self, index: usize) -> Result<[[u8; 32]; 2], RunnerError> {
    //
    // }
}