use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use guild_backend::{
    domain::{
        entities::{
            profile::Profile,
            project_like::ProjectLike,
            projects::{Project, ProjectId, ProjectStatus},
        },
        repositories::{ProfileRepository, ProjectLikeRepository, ProjectRepository},
        services::auth_service::{AuthChallenge, AuthResult, AuthService},
        value_objects::WalletAddress,
    },
    presentation::api::{test_api, AppState},
};
use std::{collections::HashSet, sync::Arc};
use tower::ServiceExt;

#[derive(Default)]
struct FakeProfileRepo;

#[async_trait::async_trait]
impl ProfileRepository for FakeProfileRepo {
    async fn find_by_address(
        &self,
        _address: &WalletAddress,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    async fn create(&self, _profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn update(&self, _profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn delete(&self, _address: &WalletAddress) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn find_by_github_login(
        &self,
        _github_login: &str,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn find_by_twitter_handle(
        &self,
        _twitter_handle: &str,
    ) -> Result<Option<Profile>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(None)
    }

    async fn get_login_nonce_by_wallet_address(
        &self,
        _address: &WalletAddress,
    ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
        Ok(None)
    }

    async fn increment_login_nonce(
        &self,
        _address: &WalletAddress,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Default)]
struct FakeAuthService;

#[async_trait::async_trait]
impl AuthService for FakeAuthService {
    async fn verify_signature(
        &self,
        _challenge: &AuthChallenge,
        _signature: &str,
    ) -> Result<Option<AuthResult>, Box<dyn std::error::Error>> {
        Ok(None)
    }
}

#[derive(Default)]
struct FakeProjectRepo {
    projects: std::sync::Mutex<HashSet<ProjectId>>,
}

#[async_trait::async_trait]
impl ProjectRepository for FakeProjectRepo {
    async fn create(&self, project: &Project) -> Result<(), Box<dyn std::error::Error>> {
        self.projects.lock().unwrap().insert(project.id);
        Ok(())
    }

    async fn find_by_id(
        &self,
        id: &ProjectId,
    ) -> Result<Option<Project>, Box<dyn std::error::Error>> {
        let exists = self.projects.lock().unwrap().contains(id);
        Ok(exists.then(|| Project::new(
            "x".into(),
            "y".into(),
            ProjectStatus::Proposal,
            WalletAddress::new("0x0000000000000000000000000000000000000000".into()).unwrap(),
        )))
    }

    async fn find_all(
        &self,
        _status: Option<ProjectStatus>,
        _creator: Option<&WalletAddress>,
        _limit: Option<i64>,
        _offset: Option<i64>,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    async fn find_by_creator(
        &self,
        _creator: &WalletAddress,
    ) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }

    async fn update(&self, _project: &Project) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    async fn delete(&self, id: &ProjectId) -> Result<(), Box<dyn std::error::Error>> {
        self.projects.lock().unwrap().remove(id);
        Ok(())
    }

    async fn exists(&self, id: &ProjectId) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.projects.lock().unwrap().contains(id))
    }

    async fn profile_exists(
        &self,
        _address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(true)
    }
}

#[derive(Default)]
struct FakeLikeRepo {
    likes: std::sync::Mutex<HashSet<(ProjectId, String)>>,
}

#[async_trait::async_trait]
impl ProjectLikeRepository for FakeLikeRepo {
    async fn create(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .likes
            .lock()
            .unwrap()
            .insert((*project_id, user_address.as_str().to_string())))
    }

    async fn delete(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self
            .likes
            .lock()
            .unwrap()
            .remove(&(*project_id, user_address.as_str().to_string())))
    }

    async fn list_by_project(
        &self,
        project_id: &ProjectId,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ProjectLike>, Box<dyn std::error::Error>> {
        let mut list: Vec<ProjectLike> = self
            .likes
            .lock()
            .unwrap()
            .iter()
            .filter(|(pid, _)| pid == project_id)
            .skip(offset as usize)
            .take(limit as usize)
            .map(|(pid, addr)| ProjectLike {
                project_id: *pid,
                user_address: WalletAddress(addr.clone()),
                created_at: chrono::Utc::now(),
            })
            .collect();
        list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(list)
    }

    async fn count_by_project(
        &self,
        project_id: &ProjectId,
    ) -> Result<i64, Box<dyn std::error::Error>> {
        Ok(self
            .likes
            .lock()
            .unwrap()
            .iter()
            .filter(|(pid, _)| pid == project_id)
            .count() as i64)
    }

    async fn exists(
        &self,
        project_id: &ProjectId,
        user_address: &WalletAddress,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.likes.lock().unwrap().contains(&(
            *project_id,
            user_address.as_str().to_string(),
        )))
    }
}

#[tokio::test]
async fn likes_crd_flow_works() {
    let project_id = ProjectId::new();

    let project_repo = Arc::new(FakeProjectRepo::default());
    project_repo.projects.lock().unwrap().insert(project_id);

    let app = test_api(AppState {
        profile_repository: Arc::new(FakeProfileRepo::default()),
        project_repository: project_repo,
        project_like_repository: Arc::new(FakeLikeRepo::default()),
        auth_service: Arc::new(FakeAuthService::default()),
    });

    let project_id_str = project_id.value().to_string();

    // Create like
    let create_req = Request::builder()
        .method("POST")
        .uri(format!("/projects/{}/likes", project_id_str))
        .header("x-eth-address", "0xABcdefabcdefabcdefabcdefabcdefabcdefabcdEF")
        .body(Body::empty())
        .unwrap();
    let create_resp = app.clone().oneshot(create_req).await.unwrap();
    assert_eq!(create_resp.status(), StatusCode::CREATED);

    // Duplicate like -> conflict
    let dup_req = Request::builder()
        .method("POST")
        .uri(format!("/projects/{}/likes", project_id_str))
        .header("x-eth-address", "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
        .body(Body::empty())
        .unwrap();
    let dup_resp = app.clone().oneshot(dup_req).await.unwrap();
    assert_eq!(dup_resp.status(), StatusCode::CONFLICT);

    // Read likes
    let read_req = Request::builder()
        .method("GET")
        .uri(format!("/projects/{}/likes", project_id_str))
        .body(Body::empty())
        .unwrap();
    let read_resp = app.clone().oneshot(read_req).await.unwrap();
    assert_eq!(read_resp.status(), StatusCode::OK);

    // Delete like
    let del_req = Request::builder()
        .method("DELETE")
        .uri(format!("/projects/{}/likes", project_id_str))
        .header("x-eth-address", "0xABCDEFabcdefabcdefabcdefabcdefabcdefabcdef")
        .body(Body::empty())
        .unwrap();
    let del_resp = app.clone().oneshot(del_req).await.unwrap();
    assert_eq!(del_resp.status(), StatusCode::NO_CONTENT);

    // Delete again -> not found
    let del2_req = Request::builder()
        .method("DELETE")
        .uri(format!("/projects/{}/likes", project_id_str))
        .header("x-eth-address", "0xabcdefabcdefabcdefabcdefabcdefabcdefabcdef")
        .body(Body::empty())
        .unwrap();
    let del2_resp = app.oneshot(del2_req).await.unwrap();
    assert_eq!(del2_resp.status(), StatusCode::NOT_FOUND);
}

