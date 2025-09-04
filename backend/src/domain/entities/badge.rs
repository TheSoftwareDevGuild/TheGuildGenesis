use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub issuer_address: String,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Badge {
    pub fn new(name: String, description: String, issuer_address: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            issuer_address,
            image_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_image_url(&mut self, image_url: Option<String>) {
        self.image_url = image_url;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadge {
    pub id: Uuid,
    pub user_id: Uuid,
    pub badge_id: Uuid,
    pub awarded_at: DateTime<Utc>,
    pub awarded_by: String, // Address of the person who awarded the badge
}

impl UserBadge {
    pub fn new(user_id: Uuid, badge_id: Uuid, awarded_by: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            badge_id,
            awarded_at: Utc::now(),
            awarded_by,
        }
    }
}

