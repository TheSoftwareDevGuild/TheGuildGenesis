use chrono::{DateTime, Utc};

use crate::domain::{entities::projects::ProjectId, value_objects::WalletAddress};

#[derive(Debug, Clone)]
pub struct ProjectLike {
    pub project_id: ProjectId,
    pub user_address: WalletAddress,
    pub created_at: DateTime<Utc>,
}
