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
        let badges = self.badge_repository.find_all_badges().await?;
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
            .find_badge_by_id(id)
            .await?
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
        
        let created_badge = self.badge_repository.create_badge(badge).await?;
        
        Ok(BadgeResponse {
            id: created_badge.id,
            name: created_badge.name,
            description: created_badge.description,
            issuer_address: created_badge.issuer_address,
            image_url: created_badge.image_url,
            created_at: created_badge.created_at.to_rfc3339(),
            updated_at: created_badge.updated_at.to_rfc3339(),
        })
    }
    
    pub async fn give_badge(&self, badge_id: Uuid, request: GiveBadgeRequest) -> Result<UserBadgeResponse, String> {
        let wallet_address = WalletAddress::new(request.profile_address)?;
        let user = self.user_repository
            .find_by_wallet_address(&wallet_address)
            .await?
            .ok_or("User not found")?;
        
        // Verify badge exists
        let badge = self.badge_repository
            .find_badge_by_id(badge_id)
            .await?
            .ok_or("Badge not found")?;
        
        let user_badge = crate::domain::entities::badge::UserBadge::new(
            user.id,
            badge_id,
            badge.issuer_address.clone(),
        );
        
        let awarded_badge = self.badge_repository.award_badge(user_badge).await?;
        
        Ok(UserBadgeResponse {
            id: awarded_badge.id,
            badge: BadgeResponse {
                id: badge.id,
                name: badge.name,
                description: badge.description,
                issuer_address: badge.issuer_address,
                image_url: badge.image_url,
                created_at: badge.created_at.to_rfc3339(),
                updated_at: badge.updated_at.to_rfc3339(),
            },
            awarded_at: awarded_badge.awarded_at.to_rfc3339(),
            awarded_by: awarded_badge.awarded_by,
        })
    }
}
