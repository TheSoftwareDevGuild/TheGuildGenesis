use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::user::User;
use crate::domain::value_objects::wallet_address::WalletAddress;

#[async_trait]
pub trait UserRepository {
    async fn create(&self, user: User) -> Result<User, String>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;
    async fn find_by_wallet_address(&self, address: &WalletAddress) -> Result<Option<User>, String>;
    async fn update(&self, user: User) -> Result<User, String>;
    async fn delete(&self, id: Uuid) -> Result<(), String>;
}
