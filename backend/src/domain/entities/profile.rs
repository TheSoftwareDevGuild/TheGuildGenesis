use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Profile {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name: None,
            description: None,
            avatar_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_info(&mut self, name: Option<String>, description: Option<String>, avatar_url: Option<String>) {
        self.name = name;
        self.description = description;
        self.avatar_url = avatar_url;
        self.updated_at = Utc::now();
    }
}
