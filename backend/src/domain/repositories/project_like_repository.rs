use async_trait::async_trait;

use crate::domain::{
    entities::{project_like::ProjectLike, projects::ProjectId},
    value_objects::WalletAddress,
};

#[async_trait]
pub trait ProjectLikeRepository: Send + Sync {
    async fn create(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn delete(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>>;

    async fn list_by_project(
        &self,
        project_id: &ProjectId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProjectLike>, Box<dyn std::error::Error>>;

    async fn count_by_project(
        &self,
        project_id: &ProjectId,
    ) -> Result<i64, Box<dyn std::error::Error>>;

    async fn exists(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
