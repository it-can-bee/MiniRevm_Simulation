use lib_utils::error::RunnerError;
use crate::execute::Execute;
use crate::storage::AccountState;
use lib_core::byte_operate::u64_to_u256_array;

/* -------------------------------------------------------------------------- */
/*                              Account state operation of EVM                */
/* -------------------------------------------------------------------------- */
pub fn get_balance(address: [u8; 20], execute: &mut Execute) -> Result<[u8; 32], RunnerError> {
    let balance = execute
        .state
        .accounts
        .get(&address)
        .map(|account| account.balance)
        .ok_or(RunnerError::AccountNotFound);
    Ok(balance?)
}

pub fn get_nonce(address: [u8; 20], execute: &mut Execute) -> Result<[u8; 32], RunnerError> {
    let nonce = execute
        .state
        .accounts
        .get(&address)
        .map(|account| account.nonce)
        .ok_or(RunnerError::AccountNotFound);
    Ok(u64_to_u256_array(nonce?))
}


pub fn init_account(address: [u8; 20], execute: &mut Execute) -> Result<(), RunnerError> {
    let account = execute.state.accounts.get_mut(&address);
    match account {
        Some(_) => Ok(()),
        None => {
            execute.state.accounts.insert(
                address,
                AccountState {
                    nonce: 0,
                    balance: [0; 32],
                    storage: std::collections::HashMap::new(),
                    code_hash: [0u8; 32],
                },
            );
            increment_nonce(address, execute)
        }
    }
}

pub fn delete_account(address: [u8; 20], execute: &mut Execute) -> Result<(), RunnerError> {
    execute.state.accounts.remove(&address);
    Ok(())
}

pub fn increment_nonce(address: [u8; 20], execute: &mut Execute) -> Result<(), RunnerError> {
    let result = execute.state.accounts.get_mut(&address);
    let nonce = match result {
        Some(account) => account,
        None => {
            println!("{:?}", RunnerError::AccountNotFound);
            return Err(RunnerError::AccountNotFound);
        }
    };
    nonce.nonce += 1;
    Ok(())
}
