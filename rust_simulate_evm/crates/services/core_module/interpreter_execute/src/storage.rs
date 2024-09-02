use lib_utils::error::RunnerError;
use ethers::utils::keccak256;
/* -------------------------------------------------------------------------- */
/*                             AccountState struct                            */
/* -------------------------------------------------------------------------- */
#[derive(Clone)]
pub struct AccountState {
    pub nonce: u64,
    pub balance: [u8; 32],
    pub storage: HashMap<[u8; 32], [u8; 32]>,    //<slot, value>
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
        // Check account
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

    // 从指定的slot读取存储值
    pub fn sload(&mut self, account: [u8; 20], slot: [u8; 32]) -> Result<[u8; 32], RunnerError> {
        //本地账户状态读取
        match self.accounts.get(&account) {
            Some(account_state) => match account_state.storage.get(&slot) {
                Some(value) => Ok(*value),
                // 存储槽为空
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
                    //获取远程节点上的存储数据
                    .block_on(future);

                match storage_result {
                    Ok(storage) => {
                        let storage_bytes = storage.to_fixed_bytes();

                        // 如果获取到的存储值是全0，存储槽为空
                        if storage_bytes == [0u8; 32] {
                            Ok([0u8; 32]) // 存储槽为空
                        } else {
                            // 保存从远程获取的存储数据到本地账户状态
                            if let Some(account_state) = self.accounts.get_mut(&account) {
                                account_state.storage.insert(slot, storage_bytes);
                            }

                            Ok(storage_bytes) // 成功获取到非空存储槽值
                        }
                    }
                    Err(_) => {
                        // 链上数据获取失败
                        Err(RunnerError::StorageRetrievalFailed)
                    }
                }
            }
        }
    }
    // 更新存储值到指定账户的slot
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

    /* -------------------------------------------------------------------------- */
    /*                             Get/put account code                            */
    /* -------------------------------------------------------------------------- */
    pub fn get_code_at(&self, address: [u8; 20]) -> Option<&Vec<u8>> {
        let account_state = self.accounts.get(&address);
        let code = if let Some(_account_state) = account_state {
            let code_hash = _account_state.code_hash;
            self.get_code(code_hash)
        } else {
            None
        };
        code
    }
    //根据hash=>code的Vec<u8>
    fn get_code(&self, code_hash: [u8; 32]) -> Option<&Vec<u8>> {
        self.codes.get(&code_hash)
    }

    //将合约代码存储在特定的账户地址
    pub fn put_code_at(&mut self, address: [u8; 20], code: Vec<u8>) -> Result<(), RunnerError> {
        let code_hash = self.put_code(code)?;

        match self.accounts.get_mut(&address) {
            Some(account_state) => {
                account_state.code_hash = code_hash.to_owned();
                Ok(())
            }
            None => Err(RunnerError::AccountNotFound),
        }
    }
    //code存储到EVM的codes哈希表
    fn put_code(&mut self, code: Vec<u8>) -> Result<[u8; 32], RunnerError> {
        // Check if static mode is enabled
        if self.static_mode {
            return Err(RunnerError::StaticCallStateChanged);
        }

        if code.is_empty() {
            return Err(RunnerError::EmptyCode);
        }

        let code_hash = keccak256(&code);
        self.codes.insert(code_hash, code);
        Ok(code_hash)
    }

    //打印EVM当前状态
    pub fn debug_state(&mut self) {
        let border_line =
            "╔═══════════════════════════════════════════════════════════════════════════════════════════════════════╗";
        let footer_line =
            "╚═══════════════════════════════════════════════════════════════════════════════════════════════════════╝";
        let separator_line =
            "╟───────────────────────────────────────────────────────────────────────────────────────────────────────╢";

        // Print out the storage header
        println!("\n{}", border_line.green());
        println!(
            "║ {:<101} ║",
            "Final EVM State".yellow().bold()
        );
        println!("{}", footer_line.green());

        // Create a vector of all addresses
        let addresses: Vec<_> = self.accounts.keys().cloned().collect();

        // Iterate over the vector of addresses
        for address in addresses {
            println!("{}", separator_line.green());

            // Print Address
            let hex_address: String = debug::to_hex_address(address.to_owned());
            println!("║ {}: {:<92} ║", "Address".cyan(), hex_address.blue());

            let account_state = &self.accounts[&address];

            // Print Nonce
            println!("║ {}: {:<100} ║", "Nonce".magenta(), account_state.nonce);

            // Print Balance
            let balance = U256::from(account_state.balance).to_string();
            println!("║ {}: {:<100} ║", "Balance".magenta(), balance.green());

            // Print Code Hash
            let code_hash = debug::to_hex_string(account_state.code_hash);
            let code_status = if account_state.code_hash == [0u8; 32] {
                "Empty code".red().to_string()
            } else {
                code_hash.yellow().to_string()
            };
            println!("║ {}: {:<100} ║", "Code Hash".magenta(), code_status);

            // Print Storage
            println!("║ {}: {:<100} ║", "Storage".magenta(), "");
            if account_state.storage.is_empty() {
                println!("║    {}: {:<96} ║", "Empty storage".red(), "");
            } else {
                for (slot, value) in &account_state.storage {
                    let slot_hex = debug::to_hex_string(slot.to_owned());
                    let value_hex = debug::to_hex_string(value.to_owned());
                    println!("║    {}: {:<88} ║", "Slot".bright_blue(), slot_hex);
                    println!("║    {}: {:<88} ║", "Value".blue(), value_hex);
                }
            }

            // If code exists, print the code
            if account_state.code_hash != [0u8; 32] {
                let code = self.get_code_at(address.to_owned()).unwrap();
                let code_hex = debug::vec_to_hex_string(code.to_owned());
                println!("║ {}: {:<100} ║", "Code".magenta(), code_hex);
            }
        }

        // Footer or message if state is empty
        if self.accounts.is_empty() {
            println!("║ {:<101} ║", "Empty EVM state".red());
        }
        println!("{}", footer_line.green());
    }

}