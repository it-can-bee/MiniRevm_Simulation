use lib_utils::error::RunnerError;
/* -------------------------------------------------------------------------- */
/*                             AccountState struct                            */
/* -------------------------------------------------------------------------- */
#[derive(Clone)]
pub struct AccountState {
    pub nonce: u64,
    pub balance: [u8; 32],
    //账户的存储空间，用hashmap表示，键(存储槽位)和值都是32字节数组
    pub storage: HashMap<[u8; 32], [u8; 32]>,
    pub code_hash: [u8; 32],
}

//Debug Display Traits
impl fmt::Debug for AccountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut code_hash: String = debug::to_hex_string(self.code_hash);
        //check空哈希
        if self.code_hash == [0u8; 32] {
            code_hash = format!("{}", "Empty code".red()).to_string()
        }
        //打印nonce
        writeln!(f, "  {}: {}", "Nonce".magenta(), self.nonce)?;
        //余额
        writeln!(
            f,
            "  {}: {}",
            "Balance".magenta(),
            U256::from(self.balance).to_string()
        )?;
        //哈希
        writeln!(f, "  {}: {}", "Code Hash".magenta(), code_hash)?;
        //存储槽和值
        write!(f, "  {}: ", "Storage".magenta())?;
        for (slot, value) in &self.storage {
            println!("\n┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
            // Print the slot
            let hex: String = debug::to_hex_string(slot.to_owned());
            println!("│ {}:  {} │", "Slot".bright_blue(), hex);

            // Print the value
            let hex: String = debug::to_hex_string(value.to_owned());
            println!("│ {}: {} │", "Value".blue(), hex);

            println!("└────────────────────────────────────────────────────────────────────────────────────────────────────────┘");
        }
        if self.storage.is_empty() {
            write!(f, "  {}", "Empty storage".red())?;
        }
        Ok(())
    }
}

/* -------------------------------------------------------------------------- */
/*                              EVM state struct                              */
/* -------------------------------------------------------------------------- */
use std::collections::HashMap;
use std::fmt;
use colored::Colorize;
use ethers::addressbook::Address;
use ethers::prelude::{H256, U256};
use ethers::prelude::*;

use lib_core::debug;
use crate::log::Log;

#[derive(Debug)]
pub struct EvmState {
    pub accounts: HashMap<[u8; 20], AccountState>,
    pub codes: HashMap<[u8; 32], Vec<u8>>,
    pub logs: Vec<Log>,
    pub static_mode: bool,     //pure view
    pub provider: Option<Provider<Http>>, //节点 缺少本地状态时从链上拉取数据
}

impl EvmState {
    pub fn new(fork_url: Option<String>) -> Self {
        Self {
            accounts: HashMap::new(),
            codes: HashMap::new(),
            logs: Vec::new(),
            static_mode: false,
            provider: if fork_url.is_some() {
                Some(Provider::<Http>::try_from(fork_url.unwrap()).unwrap())
            } else {
                None
            },
        }
    }

    pub fn transfer(
        &mut self,
        from: [u8; 20],
        to: [u8; 20],
        value: [u8; 32],
    ) -> Result<(), RunnerError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(RunnerError::StaticCallStateChanged);
        }

        let value_u256 = U256::from_big_endian(&value);

        let from_balance = U256::from_big_endian(
            &self
                .accounts
                .get(&from)
                .ok_or(RunnerError::AccountNotFound)?
                .balance,
        );

        let to_balance = U256::from_big_endian(
            &self
                .accounts
                .get(&to)
                .ok_or(RunnerError::AccountNotFound)?
                .balance,
        );

        // Check if the balance is sufficient
        if from_balance < value_u256 {
            return Err(RunnerError::InsufficientBalance);
        }

        // Transfer the value
        let new_from_balance = from_balance - value_u256;
        let new_to_balance = to_balance + value_u256;

        // update the balance of the from_account and to_account
        if let Some(from_account) = self.accounts.get_mut(&from) {
            let mut result_bytes = [0u8; 32];
            new_from_balance.to_big_endian(&mut result_bytes);
            from_account.balance = result_bytes;
        }
        if let Some(to_account) = self.accounts.get_mut(&to) {
            let mut result_bytes = [0u8; 32];
            new_to_balance.to_big_endian(&mut result_bytes);
            to_account.balance = result_bytes;
        }

        Ok(())
    }

    // 读取以太坊账户的存储槽
    pub fn sload(&mut self, account: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], RunnerError> {
        //本地账户状态读取
        match self.accounts.get(&account) {
            Some(account_state) => match account_state.storage.get(&slot) {
                Some(value) => Ok(*value),
                None => Ok([0u8; 32]),
            },
            //不在则去链上获取
            None => {
                let provider = self.provider.as_ref();
                if provider.is_none() {
                    return Ok([0u8; 32]);
                }
                //更新本地状态
                let contract_address = Address::from(account);
                let future =
                    provider
                        .unwrap()
                        .get_storage_at(contract_address, H256::from(&slot), None);

                // Block on the future and get the result
                let storage_result = tokio::runtime::Runtime::new()
                    .expect("Could not create a Runtime")
                    .block_on(future);

                match storage_result {
                    Ok(storage) => {
                        let storage_bytes = storage.to_fixed_bytes();

                        // Save the fetched storage data locally
                        if let Some(account_state) = self.accounts.get_mut(&account) {
                            account_state.storage.insert(slot, storage_bytes);
                        }

                        Ok(storage_bytes)
                    }
                    Err(_) => Ok([0u8; 32]),
                }
            }
        }
    }
    // 写入以太坊账户的存储槽
    pub fn sstore(
        &mut self,
        account: [u8; 20],
        slot: [u8; 32],
        value: [u8; 32],
    ) -> Result<(), RunnerError> {
        // 涉及状态更改 检查模式
        if self.static_mode {
            return Err(RunnerError::StaticCallStateChanged);
        }

        match self.accounts.get_mut(&account) {
            Some(account_state) => {
                account_state.storage.insert(slot, value);
                Ok(())
            }
            None => Err(RunnerError::AccountNotFound),
        }
    }
}