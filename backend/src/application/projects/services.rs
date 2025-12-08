use std::sync::Arc;

use crate::domain::{
    entities::projects::{Project, ProjectId, ProjectStatus},
    repositories::project_repository::ProjectRepository,
    value_objects::WalletAddress,
};

use super::dtos::{CreateProjectDto, ProjectDto, UpdateProjectDto};

/// Project service - handles business logic
pub struct ProjectService {
    repository: Arc<dyn ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<dyn ProjectRepository>) -> Self {
        Self { repository }
    }

    /// Create a new project
    pub async fn create_project(
        &self,
        dto: CreateProjectDto,
        creator: WalletAddress,
    ) -> Result<ProjectDto, Box<dyn std::error::Error>> {
        // Validate that creator has a profile
        if !self.repository.profile_exists(&creator).await? {
            return Err("Only addresses with profiles can create projects".into());
        }

        // Create project entity
        let mut project = Project::new(dto.name, dto.description, dto.status, creator);

        // Validate project
        project.validate()?;

        // Save to repository
        self.repository.create(&project).await?;

        Ok(ProjectDto::from(project))
    }

    /// Get a project by ID
    pub async fn get_project(
        &self,
        id: ProjectId,
    ) -> Result<Option<ProjectDto>, Box<dyn std::error::Error>> {
        let project = self.repository.find_by_id(&id).await?;
        Ok(project.map(ProjectDto::from))
    }

    /// List all projects with optional filters
    pub async fn list_projects(
        &self,
        status: Option<ProjectStatus>,
        creator: Option<WalletAddress>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<ProjectDto>, Box<dyn std::error::Error>> {
        // Validate and limit pagination
        let limit = limit.map(|l| l.max(1).min(100));
        let offset = offset.map(|o| o.max(0));

        let projects = self
            .repository
            .find_all(status, creator.as_ref(), limit, offset)
            .await?;

        Ok(projects.into_iter().map(ProjectDto::from).collect())
    }

    /// Get projects by creator
    pub async fn get_projects_by_creator(
        &self,
        creator: WalletAddress,
    ) -> Result<Vec<ProjectDto>, Box<dyn std::error::Error>> {
        let projects = self.repository.find_by_creator(&creator).await?;
        Ok(projects.into_iter().map(ProjectDto::from).collect())
    }

    /// Update a project
    pub async fn update_project(
        &self,
        id: ProjectId,
        dto: UpdateProjectDto,
        _requester: WalletAddress, // TODO: Verify requester is creator when auth is implemented
    ) -> Result<ProjectDto, Box<dyn std::error::Error>> {
        // Check if project exists
        let mut project = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or("Project not found")?;

        // TODO: Verify that requester is the creator
        // if requester != project.creator {
        //     return Err("Only the creator can update this project".into());
        // }

        // Update project
        project.update_info(dto.name, dto.description, dto.status);

        // Validate updated project
        project.validate()?;

        // Save to repository
        self.repository.update(&project).await?;

        Ok(ProjectDto::from(project))
    }

    /// Delete a project
    pub async fn delete_project(
        &self,
        id: ProjectId,
        _requester: WalletAddress, // TODO: Verify requester is creator when auth is implemented
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if project exists
        let project = self
            .repository
            .find_by_id(&id)
            .await?
            .ok_or("Project not found")?;

        // TODO: Verify that requester is the creator
        // if requester != project.creator {
        //     return Err("Only the creator can delete this project".into());
        // }

        // Delete from repository
        self.repository.delete(&project.id).await?;

        Ok(())
    }

    /// Check if a project exists
    pub async fn project_exists(
        &self,
        id: ProjectId,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.repository.exists(&id).await
    }
}