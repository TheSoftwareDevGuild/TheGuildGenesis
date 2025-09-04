use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WalletAddress(String);

impl WalletAddress {
    pub fn new(address: String) -> Result<Self, String> {
        if address.is_empty() {
            return Err("Wallet address cannot be empty".to_string());
        }
        
        // Basic validation - should be a valid Ethereum address format
        if !address.starts_with("0x") || address.len() != 42 {
            return Err("Invalid wallet address format".to_string());
        }
        
        Ok(Self(address))
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for WalletAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<WalletAddress> for String {
    fn from(addr: WalletAddress) -> Self {
        addr.0
    }
}
