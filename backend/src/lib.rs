pub mod domain;
pub mod application;
// pub mod infrastructure; // Temporarily disabled to avoid SQLx compilation issues

use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

// Database repositories are temporarily disabled
// use crate::infrastructure::repositories::{
//     PostgresUserRepository, PostgresProfileRepository, PostgresBadgeRepository,
// };
use crate::application::services::{
    AuthApplicationService, ProfileApplicationService, BadgeApplicationService,
};

pub async fn create_app(_pool: sqlx::PgPool) -> Router {
    // For now, create mock repositories that don't use the database
    // TODO: Replace with real database implementations
    let user_repository = Box::new(MockUserRepository);
    let profile_repository = Box::new(MockProfileRepository);
    let badge_repository = Box::new(MockBadgeRepository);
    
    // Create application services
    let auth_service = AuthApplicationService::new(user_repository.clone());
    let profile_service = ProfileApplicationService::new(user_repository.clone(), profile_repository);
    let badge_service = BadgeApplicationService::new(user_repository, badge_repository);
    
    Router::new()
        .route("/health", get(health_check))
        .route("/siwe/nonce", get(handle_get_nonce))
        .route("/siwe/verify", post(handle_verify_message))
        .route("/profiles", get(handle_get_profiles))
        .route("/profiles", post(handle_create_profile))
        .route("/profiles/:address", get(handle_get_profile))
        .route("/profiles/:address", put(handle_update_profile))
        .route("/badges", get(handle_get_badges))
        .route("/badges", post(handle_create_badge))
        .route("/badges/:id", get(handle_get_badge))
        .route("/badges/:id/give", post(handle_give_badge))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin(Any)
                        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                        .allow_headers(Any),
                )
                .layer(DefaultBodyLimit::max(1024 * 1024)), // 1MB limit
        )
        .with_state(AppState {
            auth_service,
            profile_service,
            badge_service,
        })
}

#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthApplicationService,
    pub profile_service: ProfileApplicationService,
    pub badge_service: BadgeApplicationService,
}

async fn health_check() -> &'static str {
    "OK"
}

// Mock repository implementations for development
// use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::entities::{user::User, profile::Profile, badge::Badge};
use crate::domain::repositories::{user_repository::UserRepository, profile_repository::ProfileRepository, badge_repository::BadgeRepository};

#[derive(Clone)]
struct MockUserRepository;

#[async_trait::async_trait]
impl UserRepository for MockUserRepository {
    async fn create(&self, _user: &User) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn find_by_id(&self, _id: &Uuid) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    async fn find_by_wallet_address(&self, _address: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    async fn update(&self, _user: &User) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn delete(&self, _id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Clone)]
struct MockProfileRepository;

#[async_trait::async_trait]
impl ProfileRepository for MockProfileRepository {
    async fn create(&self, _profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn find_by_id(&self, _id: &Uuid) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    async fn find_by_user_id(&self, _user_id: &Uuid) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
    
    async fn update(&self, _profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn delete(&self, _id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

#[derive(Clone)]
struct MockBadgeRepository;

#[async_trait::async_trait]
impl BadgeRepository for MockBadgeRepository {
    async fn create(&self, _badge: &Badge) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn find_by_id(&self, _id: &Uuid) -> Result<Option<Badge>, Box<dyn std::error::Error>> {
        Ok(None)
    }
    
    async fn find_all(&self) -> Result<Vec<Badge>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
    
    async fn update(&self, _badge: &Badge) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn delete(&self, _id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn award_badge(&self, _user_badge: &crate::domain::repositories::badge_repository::UserBadge) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
    
    async fn find_user_badges(&self, _user_id: &Uuid) -> Result<Vec<crate::domain::repositories::badge_repository::UserBadge>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
    
    async fn find_badge_users(&self, _badge_id: &Uuid) -> Result<Vec<crate::domain::repositories::badge_repository::UserBadge>, Box<dyn std::error::Error>> {
        Ok(vec![])
    }
}

// Handler functions will be implemented in separate files
async fn handle_get_nonce() -> &'static str {
    "Not implemented yet"
}

async fn handle_verify_message() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_profiles() -> &'static str {
    "Not implemented yet"
}

async fn handle_create_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_update_profile() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_badges() -> &'static str {
    "Not implemented yet"
}

async fn handle_create_badge() -> &'static str {
    "Not implemented yet"
}

async fn handle_get_badge() -> &'static str {
    "Not implemented yet"
}

async fn handle_give_badge() -> &'static str {
    "Not implemented yet"
}