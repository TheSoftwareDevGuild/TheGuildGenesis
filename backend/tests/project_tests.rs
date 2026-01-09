#[cfg(test)]
mod project_tests {
    use guild_backend::application::commands::create_project::create_project;
    use guild_backend::application::commands::delete_project::delete_project;
    use guild_backend::application::commands::update_project::update_project;
    use guild_backend::application::dtos::project_dtos::{
        CreateProjectRequest, UpdateProjectRequest,
    };
    use guild_backend::application::queries::get_project::get_project;
    use guild_backend::domain::entities::projects::{Project, ProjectId, ProjectStatus};
    use guild_backend::domain::repositories::project_repository::ProjectRepository;
    use guild_backend::domain::value_objects::WalletAddress;
    use std::sync::Arc;

    // A fake in-memory repository for testing
    struct FakeProjectRepo {
        projects: std::sync::Mutex<Vec<Project>>,
    }

    #[async_trait::async_trait]
    impl ProjectRepository for FakeProjectRepo {
        async fn create(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
            let mut list = self.projects.lock().unwrap();
            list.push(project.clone());
            Ok(())
        }

        async fn find_by_id(
            &self,
            id: &ProjectId,
        ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
            let list = self.projects.lock().unwrap();
            Ok(list.iter().find(|p| p.id == *id).cloned())
        }

        async fn find_all(
            &self,
            status: Option<ProjectStatus>,
            creator: Option<&WalletAddress>,
            _limit: Option<i64>,
            _offset: Option<i64>,
        ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
            let list = self.projects.lock().unwrap();
            let filtered: Vec<Project> = list
                .iter()
                .filter(|p| {
                    let status_match = status.map(|s| p.status == s).unwrap_or(true);
                    let creator_match = creator.map(|c| &p.creator == c).unwrap_or(true);
                    status_match && creator_match
                })
                .cloned()
                .collect();
            Ok(filtered)
        }

        async fn find_by_creator(
            &self,
            creator: &WalletAddress,
        ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
            let list = self.projects.lock().unwrap();
            Ok(list
                .iter()
                .filter(|p| &p.creator == creator)
                .cloned()
                .collect())
        }

        async fn update(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
            let mut list = self.projects.lock().unwrap();
            if let Some(slot) = list.iter_mut().find(|p| p.id == project.id) {
                *slot = project.clone();
                Ok(())
            } else {
                Err("Project not found".into())
            }
        }

        async fn delete(&self, id: &ProjectId) -> Result<(), Box<dyn std::error::Error>> {
            let mut list = self.projects.lock().unwrap();
            list.retain(|p| &p.id != id);
            Ok(())
        }

        async fn exists(&self, id: &ProjectId) -> Result<bool, Box<dyn std::error::Error>> {
            let list = self.projects.lock().unwrap();
            Ok(list.iter().any(|p| &p.id == id))
        }

        async fn profile_exists(
            &self,
            _address: &WalletAddress,
        ) -> Result<bool, Box<dyn std::error::Error>> {
            // For testing, assume profile always exists
            Ok(true)
        }
    }

    #[tokio::test]
    async fn create_project_succeeds() {
        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![]),
        });

        let creator_address = "0x1234567890123456789012345678901234567890".to_string();

        let req = CreateProjectRequest {
            name: "Test Project".into(),
            description: "A test project".into(),
            status: ProjectStatus::Proposal,
        };

        let result = create_project(repo.clone(), creator_address.clone(), req).await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.name, "Test Project");
        assert_eq!(response.creator, creator_address);
    }

    #[tokio::test]
    async fn create_project_validates_name() {
        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![]),
        });

        let creator_address = "0x1234567890123456789012345678901234567890".to_string();

        // Empty name should fail
        let req = CreateProjectRequest {
            name: "".into(),
            description: "Description".into(),
            status: ProjectStatus::Proposal,
        };

        let result = create_project(repo.clone(), creator_address, req).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("empty"));
    }

    #[tokio::test]
    async fn update_project_by_creator_succeeds() {
        let creator =
            WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();

        let project = Project::new(
            "Original Name".into(),
            "Original Description".into(),
            ProjectStatus::Proposal,
            creator.clone(),
        );
        let project_id = project.id;

        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![project]),
        });

        let req = UpdateProjectRequest {
            name: Some("Updated Name".into()),
            description: None,
            status: Some(ProjectStatus::Ongoing),
        };

        let result = update_project(
            repo.clone(),
            creator.to_string(),
            project_id.value().to_string(),
            req,
        )
        .await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "Updated Name");
        assert_eq!(response.status, ProjectStatus::Ongoing);
    }

    #[tokio::test]
    async fn update_project_by_non_creator_fails() {
        let creator =
            WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        let other_user =
            WalletAddress::new("0x0987654321098765432109876543210987654321".to_string()).unwrap();

        let project = Project::new(
            "Project".into(),
            "Description".into(),
            ProjectStatus::Proposal,
            creator,
        );
        let project_id = project.id;

        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![project]),
        });

        let req = UpdateProjectRequest {
            name: Some("Hacked Name".into()),
            description: None,
            status: None,
        };

        let result = update_project(
            repo.clone(),
            other_user.to_string(),
            project_id.value().to_string(),
            req,
        )
        .await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Only the creator"));
    }

    #[tokio::test]
    async fn delete_project_by_creator_succeeds() {
        let creator =
            WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();

        let project = Project::new(
            "To Delete".into(),
            "Description".into(),
            ProjectStatus::Proposal,
            creator.clone(),
        );
        let project_id = project.id;

        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![project]),
        });

        let result = delete_project(
            repo.clone(),
            creator.to_string(),
            project_id.value().to_string(),
        )
        .await;

        assert!(result.is_ok());

        // Verify it's deleted
        let exists = repo.exists(&project_id).await.unwrap();
        assert!(!exists);
    }

    #[tokio::test]
    async fn delete_project_by_non_creator_fails() {
        let creator =
            WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();
        let other_user =
            WalletAddress::new("0x0987654321098765432109876543210987654321".to_string()).unwrap();

        let project = Project::new(
            "Protected".into(),
            "Description".into(),
            ProjectStatus::Proposal,
            creator,
        );
        let project_id = project.id;

        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![project]),
        });

        let result = delete_project(
            repo.clone(),
            other_user.to_string(),
            project_id.value().to_string(),
        )
        .await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Only the creator"));
    }

    #[tokio::test]
    async fn get_project_by_id_succeeds() {
        let creator =
            WalletAddress::new("0x1234567890123456789012345678901234567890".to_string()).unwrap();

        let project = Project::new(
            "Findable Project".into(),
            "Description".into(),
            ProjectStatus::Ongoing,
            creator,
        );
        let project_id = project.id;

        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![project.clone()]),
        });

        let result = get_project(repo.clone(), project_id.value().to_string()).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.name, "Findable Project");
        assert_eq!(response.status, ProjectStatus::Ongoing);
    }

    #[tokio::test]
    async fn get_nonexistent_project_fails() {
        let repo = Arc::new(FakeProjectRepo {
            projects: std::sync::Mutex::new(vec![]),
        });

        let fake_id = uuid::Uuid::new_v4().to_string();
        let result = get_project(repo.clone(), fake_id).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("not found"));
    }
}
