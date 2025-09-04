use std::sync::Arc;
use uuid::Uuid;
use crate::domain::repositories::{user_repository::UserRepository, badge_repository::BadgeRepository};
use crate::domain::value_objects::wallet_address::WalletAddress;
use crate::application::dtos::badge_dtos::{
    BadgeResponse, CreateBadgeRequest, GiveBadgeRequest, BadgeListResponse, UserBadgeResponse
};

#[derive(Clone)]
pub struct BadgeApplicationService {
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    badge_repository: Arc<dyn BadgeRepository + Send + Sync>,
}

impl BadgeApplicationService {
    pub fn new(
        user_repository: Box<dyn UserRepository + Send + Sync>,
        badge_repository: Box<dyn BadgeRepository + Send + Sync>,
    ) -> Self {
        Self {
            user_repository: Arc::from(user_repository),
            badge_repository: Arc::from(badge_repository),
        }
    }
    
    pub async fn get_badges(&self) -> Result<BadgeListResponse, String> {
        let badges = self.badge_repository.find_all().await.map_err(|e| e.to_string())?;
        let badge_responses: Vec<BadgeResponse> = badges
            .into_iter()
            .map(|badge| BadgeResponse {
                id: badge.id,
                name: badge.name,
                description: badge.description,
                issuer_address: badge.issuer_address,
                image_url: badge.image_url,
                created_at: badge.created_at.to_rfc3339(),
                updated_at: badge.updated_at.to_rfc3339(),
            })
            .collect();
        
        Ok(BadgeListResponse {
            badges: badge_responses,
        })
    }
    
    pub async fn get_badge(&self, id: Uuid) -> Result<BadgeResponse, String> {
        let badge = self.badge_repository
            .find_by_id(&id)
            .await.map_err(|e| e.to_string())?
            .ok_or("Badge not found")?;
        
        Ok(BadgeResponse {
            id: badge.id,
            name: badge.name,
            description: badge.description,
            issuer_address: badge.issuer_address,
            image_url: badge.image_url,
            created_at: badge.created_at.to_rfc3339(),
            updated_at: badge.updated_at.to_rfc3339(),
        })
    }
    
    pub async fn create_badge(&self, request: CreateBadgeRequest) -> Result<BadgeResponse, String> {
        let mut badge = crate::domain::entities::badge::Badge::new(
            request.name,
            request.description,
            request.issuer_address,
        );
        
        if let Some(image_url) = request.image_url {
            badge.set_image_url(Some(image_url));
        }
        
        self.badge_repository.create(&badge).await.map_err(|e| e.to_string())?;
        
        Ok(BadgeResponse {
            id: badge.id,
            name: badge.name,
            description: badge.description,
            issuer_address: badge.issuer_address,
            image_url: badge.image_url,
            created_at: badge.created_at.to_rfc3339(),
            updated_at: badge.updated_at.to_rfc3339(),
        })
    }
    
    pub async fn give_badge(&self, badge_id: Uuid, request: GiveBadgeRequest) -> Result<UserBadgeResponse, String> {
        let wallet_address = WalletAddress::new(request.profile_address).map_err(|e| e.to_string())?;
        let user = self.user_repository
            .find_by_wallet_address(&wallet_address.to_string())
            .await.map_err(|e| e.to_string())?
            .ok_or("User not found")?;
        
        // Verify badge exists
        let badge = self.badge_repository
            .find_by_id(&badge_id)
            .await.map_err(|e| e.to_string())?
            .ok_or("Badge not found")?;
        
        let user_badge = crate::domain::repositories::badge_repository::UserBadge {
            id: uuid::Uuid::new_v4(),
            user_id: user.id,
            badge_id,
            awarded_at: chrono::Utc::now(),
            awarded_by: badge.issuer_address.clone(),
        };
        
        self.badge_repository.award_badge(&user_badge).await.map_err(|e| e.to_string())?;
        
        Ok(UserBadgeResponse {
            id: user_badge.id,
            badge: BadgeResponse {
                id: badge.id,
                name: badge.name,
                description: badge.description,
                issuer_address: badge.issuer_address,
                image_url: badge.image_url,
                created_at: badge.created_at.to_rfc3339(),
                updated_at: badge.updated_at.to_rfc3339(),
            },
            awarded_at: user_badge.awarded_at.to_rfc3339(),
            awarded_by: user_badge.awarded_by,
        })
    }
}
