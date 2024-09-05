use std::fmt;

#[derive(Debug)]
pub enum RunnerError {
    // Memory errors
    OutOfBoundsByteCode,

    // System errors
    OutOfGas,
    StorageRetrievalFailed,
    EmptyCode,

    // Account errors
    AccountNotFound,
    CodeNotFound,
    EmptyByteCode,
    InsufficientBalance,
    OperationNotAllowed,

    // Flow errors
    StaticCallStateChanged,
    InvalidOpcode(u8),
    InvalidJumpDestination,

    // Stack errors
    StackTooSmall,
    StackTooDeep,
    StackOverflow,

    // General execution errors
    Revert(Vec<u8>),
    RevertWithoutData,
    NotImplemented(u8),
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RunnerError::OutOfBoundsByteCode => {
                write!(f, "Attempted to access out of bounds bytecode bytes")
            }
            RunnerError::EmptyByteCode => write!(f, "Attempted to interpret empty bytecode"),
            RunnerError::StackTooSmall => write!(f, "Attempted to read out of stacks bounds"),
            RunnerError::StackTooDeep => {
                write!(f, "Stack too deep. Maximum stack size is 1024 words")
            }
            RunnerError::AccountNotFound => {
                write!(f, "Trying to access non-existent account state")
            }
            RunnerError::CodeNotFound => write!(f, "Trying to access non-existent account code"),
            RunnerError::RevertWithoutData => write!(f, "Execution revert without data"),
            RunnerError::InsufficientBalance => write!(f, "Insufficient balance to transfer"),
            RunnerError::InvalidOpcode(op_code) => {
                write!(f, "Invalid op code 0x{:X}", op_code)
            }
            RunnerError::StaticCallStateChanged => {
                write!(f, "State changed during a static call")
            }
            RunnerError::NotImplemented(op_code) => {
                write!(f, "Op code 0x{:X} not implemented", op_code)
            }
            RunnerError::InvalidJumpDestination => write!(f, "Invalid jump destination"),
            RunnerError::Revert(data) => {
                let hex = super::debug::vec_to_hex_string(data.to_owned());
                write!(f, "Execution revert with data: {}", hex)
            },
            RunnerError::OutOfGas => write!(f, "OutOfGas to call function"),
            RunnerError::StorageRetrievalFailed => write!(f, "StorageRetrievalFailed"),
            RunnerError::EmptyCode => write!(f, " EmptyCode"),
            RunnerError::OperationNotAllowed => write!(f, "OperationNotAllowed"),
            RunnerError::StackOverflow => write!(f, "StackOverflow"),
        }
    }
}

impl std::error::Error for RunnerError {}

impl PartialEq for RunnerError {
    fn eq(&self, other: &Self) -> bool {
        use RunnerError::*;
        match (self, other) {
            (OutOfBoundsByteCode, OutOfBoundsByteCode)
            | (AccountNotFound, AccountNotFound)
            | (CodeNotFound, CodeNotFound)
            | (EmptyByteCode, EmptyByteCode)
            | (InsufficientBalance, InsufficientBalance)
            | (StaticCallStateChanged, StaticCallStateChanged)
            | (StackTooSmall, StackTooSmall)
            | (InvalidJumpDestination, InvalidJumpDestination)
            | (StackTooDeep, StackTooDeep)
            | (RevertWithoutData, RevertWithoutData) => true,
            (InvalidOpcode(a), InvalidOpcode(b)) => a == b,
            (NotImplemented(a), NotImplemented(b)) => a == b,
            (Revert(a), Revert(b)) => a == b,
            _ => false,
        }
    }
}
