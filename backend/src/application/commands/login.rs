use crate::infrastructure::jwt::JwtManager;

pub async fn login(address: String) -> Result<String, String> {
    let jwt_manager = JwtManager::new();
    jwt_manager.generate_token(&address)
}
