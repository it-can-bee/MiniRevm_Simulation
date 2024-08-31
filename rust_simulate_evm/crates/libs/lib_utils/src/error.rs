pub enum RunnerError {
    // Memory errors
    OutOfBoundsByteCode,

    // System errors
    OutOfGas,

    // Account errors
    AccountNotFound,
    CodeNotFound,
    EmptyByteCode,
    InsufficientBalance,

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