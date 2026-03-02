use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterDistributionItem {
    pub address: String,
    #[serde(rename = "badgeName")]
    pub badge_name: String,
    #[serde(rename = "distributionId")]
    pub distribution_id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegisterDistributionRequest {
    pub distributions: Vec<RegisterDistributionItem>,
}
