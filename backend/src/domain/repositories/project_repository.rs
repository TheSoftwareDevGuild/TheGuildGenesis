use async_trait::async_trait;

use crate::domain::{
    entities::projects::{Project, ProjectId, ProjectStatus},
    value_objects::WalletAddress,
};

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Create a new project
    async fn create(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;

    /// Find a project by ID
    async fn find_by_id(
        &self,
        id: &ProjectId,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>>;

    /// Find all projects with optional filters
    async fn find_all(
        &self,
        status: Option<ProjectStatus>,
        creator: Option<&WalletAddress>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>>;

    /// Find projects by creator
    async fn find_by_creator(
        &self,
        creator: &WalletAddress,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>>;

    /// Update a project
    async fn update(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>>;

    /// Delete a project
    async fn delete(&self, id: &ProjectId) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if a project exists
    async fn exists(&self, id: &ProjectId) -> Result<bool, Box<dyn std::error::Error>>;

    /// Check if a profile exists (for creator validation)
    async fn profile_exists(
        &self,
        address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>>;
}
