use super::utils::error::RunnerError;
use crate::evm_core::utils;
use colored::Colorize;
use primitive_types::U256;
use std::fmt;

pub const STACK_LIMIT:usize = 1024;
#[derive(Debug)]
pub struct Stack {
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

    pub fn swap(&mut self, index: usize) -> Result<[[u8; 32]; 2], RunnerError> {
        let len = self.stack.len();
        if index == 0 || index >= len {
            return Err(RunnerError::StackTooSmall);
        }
        //swap for index with the topStack
        self.stack.swap(len - 1, len -1 - index);
        let new_top = self.stack[len - 1];
        let old_top = self.stack[len - 1 - index];

        Ok([new_top, old_top])
    }

    pub fn dup(&mut self, index: usize) -> Result<[u8; 32], RunnerError> {
        if index == 0 || index > self.stack.len() {
            return Err(RunnerError::StackTooSmall);
        }
        let idx = self.stack.len() - index;
        let double = self.stack[idx];
        self.stack.push(double);

        Ok(double)
    }
}

impl Clone for Stack {
    /// Returns a new instance of `Stack` with the same elements as `self`.
    fn clone(&self) -> Self {
        Self {
            stack: self.stack.clone(),
        }
    }
}