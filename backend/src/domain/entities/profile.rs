use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::domain::value_objects::WalletAddress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub address: WalletAddress,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub github_login: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Profile {
    pub fn new(wallet_address: WalletAddress) -> Self {
        let now = Utc::now();
        Self {
            address: wallet_address,
            name: None,
            description: None,
            avatar_url: None,
            github_login: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_info(
        &mut self,
        name: Option<String>,
        description: Option<String>,
        avatar_url: Option<String>,
    ) {
        self.name = name;
        self.description = description;
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }
}
