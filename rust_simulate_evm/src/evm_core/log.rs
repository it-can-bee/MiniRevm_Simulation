/*
日志分为主题（topic）和数据（data）
topic是事件签名的哈希值，data是由indexed修饰的事件参数
指令LOG0到LOG4的区别在于它们包含的主题数量.LOG0没有主题，而LOG4有四个主题
*/
use std::fmt;
use colored::Colorize;

use super::utils::debug;

/* -------------------------------------------------------------------------- */
/*                                 Log struct                                 */
/* -------------------------------------------------------------------------- */
/// Represents a log entry in the Ethereum Virtual Machine (EVM) state.
pub struct Log {
    /// The address of the contract that generated the log.
    pub address: [u8; 20],
    /// The topics associated with the log.
    pub topics: Vec<[u8; 32]>,
    /// The data associated with the log.
    pub data: Vec<u8>,
}

impl fmt::Debug for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}: {}",
            "Address".magenta(),
            debug::to_hex_address(self.address)
        )?;

        write!(f, "{}: ", "Topics".magenta())?;
        if !self.topics.is_empty() {
            for (idx, topic) in self.topics.iter().enumerate() {
                println!("\n┌────────────────────────────────────────────────────────────────────────────────────────────────────────┐");
                let hex: String = debug::to_hex_string(topic.to_owned());
                println!("│ {}: {} {} │", "Topic".bright_blue(), idx, hex);
                println!("└────────────────────────────────────────────────────────────────────────────────────────────────────────┘");
            }
        } else {
            writeln!(f, "{}", "No topics".red())?;
        }

        writeln!(
            f,
            "{}: {}",
            "Data".magenta(),
            debug::vec_to_hex_string(self.data.clone())
        )?;

        Ok(())
    }
}