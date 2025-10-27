#[cfg(test)]
mod github_handle_tests {
    use guild_backend::application::commands::update_profile::update_profile;
    use guild_backend::application::dtos::profile_dtos::UpdateProfileRequest;
    use guild_backend::domain::entities::profile::Profile;
    use guild_backend::domain::repositories::profile_repository::ProfileRepository;
    use guild_backend::domain::value_objects::WalletAddress;
    use std::sync::Arc;

    // A fake in-memory repository for testing
    struct FakeRepo {
        profiles: std::sync::Mutex<Vec<Profile>>,
    }

    #[async_trait::async_trait]
    impl ProfileRepository for FakeRepo {
        async fn find_by_address(
            &self,
            address: &WalletAddress,
        ) -> Result<Option<Profile>, Box<dyn std::error::Error>> {
            let list = self.profiles.lock().unwrap();
            Ok(list.iter().find(|&p| p.address == *address).cloned())
        }

        async fn find_all(&self) -> Result<Vec<Profile>, Box<dyn std::error::Error>> {
            let list = self.profiles.lock().unwrap();
            Ok(list.clone())
        }

        async fn create(&self, _profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
            unimplemented!()
        }

        async fn update(&self, profile: &Profile) -> Result<(), Box<dyn std::error::Error>> {
            let mut list = self.profiles.lock().unwrap();
            if let Some(slot) = list.iter_mut().find(|p| p.address == profile.address) {
                *slot = profile.clone();
                Ok(())
            } else {
                Err("Not found".into())
            }
        }

        async fn delete(&self, _address: &WalletAddress) -> Result<(), Box<dyn std::error::Error>> {
            unimplemented!()
        }

        async fn find_by_github_login(
            &self,
            github_login: &str,
        ) -> Result<Option<Profile>, Box<dyn std::error::Error + Send + Sync>> {
            let lower = github_login.to_lowercase();
            let list = self.profiles.lock().unwrap();
            Ok(list
                .iter()
                .find(|&p| {
                    p.github_login
                        .as_ref()
                        .is_some_and(|h| h.to_lowercase() == lower)
                })
                .cloned())
        }

        async fn get_login_nonce_by_wallet_address(
            &self,
            _address: &WalletAddress,
        ) -> Result<Option<i64>, Box<dyn std::error::Error>> {
            Ok(Some(1))
        }

        async fn increment_login_nonce(
            &self,
            _address: &WalletAddress,
        ) -> Result<(), Box<dyn std::error::Error>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn valid_github_handle_succeeds() {
        // Setup repo with a user
        let profile = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567890".to_string())
                .unwrap(),
            name: Some("Alice".into()),
            description: None,
            avatar_url: None,
            github_login: None,
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let repo = Arc::new(FakeRepo {
            profiles: std::sync::Mutex::new(vec![profile.clone()]),
        });

        // Try updating with a valid handle
        let req = UpdateProfileRequest {
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("GitUser123".into()),
        };

        let result = update_profile(repo.clone(), profile.address.to_string(), req).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.github_login.unwrap(), "GitUser123");
    }

    #[tokio::test]
    async fn invalid_format_rejected() {
        let profile = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567891".to_string())
                .unwrap(),
            name: None,
            description: None,
            avatar_url: None,
            github_login: None,
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let repo = Arc::new(FakeRepo {
            profiles: std::sync::Mutex::new(vec![profile.clone()]),
        });

        // Try invalid handle (has @)
        let req = UpdateProfileRequest {
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("bad@name".into()),
        };

        let err = update_profile(repo.clone(), profile.address.to_string(), req).await;
        assert!(err.is_err());
        let err_msg = err.unwrap_err();
        assert!(err_msg.contains("Invalid GitHub handle format"));
    }

    #[tokio::test]
    async fn conflict_rejected_case_insensitive() {
        // Two profiles in the repo
        let profile1 = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567892".to_string())
                .unwrap(),
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("Alice".into()),
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let profile2 = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567893".to_string())
                .unwrap(),
            name: None,
            description: None,
            avatar_url: None,
            github_login: None,
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let repo = Arc::new(FakeRepo {
            profiles: std::sync::Mutex::new(vec![profile1.clone(), profile2.clone()]),
        });

        // Try to claim "alice" from profile2 (lowercase) → conflict
        let req = UpdateProfileRequest {
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("alice".into()),
        };

        let err = update_profile(repo.clone(), profile2.address.to_string(), req).await;
        assert!(err.is_err());
        let err_msg = err.unwrap_err();
        assert!(err_msg.contains("GitHub handle already taken"));
    }

    #[tokio::test]
    async fn empty_github_handle_allowed() {
        let profile = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567894".to_string())
                .unwrap(),
            name: Some("Bob".into()),
            description: None,
            avatar_url: None,
            github_login: Some("BobUser".into()),
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let repo = Arc::new(FakeRepo {
            profiles: std::sync::Mutex::new(vec![profile.clone()]),
        });

        // Try updating to empty github handle
        let req = UpdateProfileRequest {
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("".into()),
        };

        let result = update_profile(repo.clone(), profile.address.to_string(), req).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.github_login.is_none()); // Should be None, not Some("")
    }

    #[tokio::test]
    async fn user_can_update_own_github_handle() {
        let profile = Profile {
            address: WalletAddress::new("0x1234567890123456789012345678901234567895".to_string())
                .unwrap(),
            name: Some("Charlie".into()),
            description: None,
            avatar_url: None,
            github_login: Some("CharlieGit".into()),
            login_nonce: 1,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let repo = Arc::new(FakeRepo {
            profiles: std::sync::Mutex::new(vec![profile.clone()]),
        });

        // Try updating with the same handle (should succeed)
        let req = UpdateProfileRequest {
            name: None,
            description: None,
            avatar_url: None,
            github_login: Some("CharlieGit".into()),
        };

        let result = update_profile(repo.clone(), profile.address.to_string(), req).await;
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert_eq!(resp.github_login.unwrap(), "CharlieGit");
    }
}
