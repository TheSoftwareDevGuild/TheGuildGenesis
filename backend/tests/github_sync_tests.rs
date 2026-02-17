#[cfg(test)]
mod github_sync_tests {
    use async_trait::async_trait;
    use std::sync::Arc;

    use guild_backend::application::commands::sync_github_issues::{
        derive_points, sync_github_issues, transform_issue,
    };
    use guild_backend::domain::entities::github_issue::GithubIssue;
    use guild_backend::domain::repositories::github_issue_repository::GithubIssueRepository;
    use guild_backend::domain::services::github_service::{
        GitHubApiIssue, GitHubApiLabel, GitHubApiUser, GithubService,
    };

    // ========================================================================
    // Fake implementations for testing
    // ========================================================================

    struct FakeGithubIssueRepo {
        issues: std::sync::Mutex<Vec<GithubIssue>>,
    }

    #[async_trait]
    impl GithubIssueRepository for FakeGithubIssueRepo {
        async fn upsert(&self, issue: &GithubIssue) -> Result<(), Box<dyn std::error::Error>> {
            let mut list = self.issues.lock().unwrap();
            // Upsert: replace if exists, otherwise insert
            if let Some(existing) = list
                .iter_mut()
                .find(|i| i.repo_id == issue.repo_id && i.github_issue_id == issue.github_issue_id)
            {
                *existing = issue.clone();
            } else {
                list.push(issue.clone());
            }
            Ok(())
        }

        async fn find_by_key(
            &self,
            repo_id: i64,
            github_issue_id: i64,
        ) -> Result<Option<GithubIssue>, Box<dyn std::error::Error>> {
            let list = self.issues.lock().unwrap();
            Ok(list
                .iter()
                .find(|i| i.repo_id == repo_id && i.github_issue_id == github_issue_id)
                .cloned())
        }

        async fn list_by_repo(
            &self,
            repo: &str,
            state: Option<&str>,
        ) -> Result<Vec<GithubIssue>, Box<dyn std::error::Error>> {
            let list = self.issues.lock().unwrap();
            Ok(list
                .iter()
                .filter(|i| i.repo == repo && state.is_none_or(|s| i.state == s))
                .cloned()
                .collect())
        }
    }

    struct FakeGithubService {
        issues: Vec<GitHubApiIssue>,
        repo_id: i64,
    }

    #[async_trait]
    impl GithubService for FakeGithubService {
        async fn fetch_issues(
            &self,
            _repo: &str,
            _since: Option<&str>,
        ) -> Result<(i64, Vec<GitHubApiIssue>), Box<dyn std::error::Error>> {
            Ok((self.repo_id, self.issues.clone()))
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn make_api_issue(
        id: i64,
        number: i32,
        title: &str,
        state: &str,
        labels: Vec<&str>,
        assignees: Vec<&str>,
        is_pr: bool,
        closed_at: Option<&str>,
    ) -> GitHubApiIssue {
        GitHubApiIssue {
            id,
            number,
            title: title.to_string(),
            state: state.to_string(),
            html_url: format!("https://github.com/test/repo/issues/{number}"),
            labels: labels
                .into_iter()
                .map(|l| GitHubApiLabel {
                    name: l.to_string(),
                })
                .collect(),
            assignees: assignees
                .into_iter()
                .map(|a| GitHubApiUser {
                    login: a.to_string(),
                })
                .collect(),
            created_at: "2025-01-01T00:00:00Z".to_string(),
            closed_at: closed_at.map(|s| s.to_string()),
            updated_at: "2025-01-02T00:00:00Z".to_string(),
            pull_request: if is_pr {
                Some(serde_json::json!({}))
            } else {
                None
            },
        }
    }

    // ========================================================================
    // Test: derive_points from labels
    // ========================================================================

    #[test]
    fn test_derive_points_with_points_label() {
        let labels = vec![
            GitHubApiLabel {
                name: "bug".to_string(),
            },
            GitHubApiLabel {
                name: "points:3".to_string(),
            },
        ];
        assert_eq!(derive_points(&labels), 3);
    }

    #[test]
    fn test_derive_points_case_insensitive() {
        let labels = vec![GitHubApiLabel {
            name: "Points:5".to_string(),
        }];
        assert_eq!(derive_points(&labels), 5);
    }

    #[test]
    fn test_derive_points_no_match_defaults_to_zero() {
        let labels = vec![
            GitHubApiLabel {
                name: "bug".to_string(),
            },
            GitHubApiLabel {
                name: "enhancement".to_string(),
            },
        ];
        assert_eq!(derive_points(&labels), 0);
    }

    #[test]
    fn test_derive_points_empty_labels() {
        let labels: Vec<GitHubApiLabel> = vec![];
        assert_eq!(derive_points(&labels), 0);
    }

    // ========================================================================
    // Test: transform_issue normalizes labels to lower-case
    // ========================================================================

    #[test]
    fn test_transform_issue_normalizes_labels() {
        let api_issue = make_api_issue(
            1,
            1,
            "Test Issue",
            "open",
            vec!["Bug", "Points:3"],
            vec!["alice"],
            false,
            None,
        );

        let result = transform_issue("org/repo", 42, &api_issue).unwrap();

        let labels = result.labels.as_array().unwrap();
        assert_eq!(labels[0].as_str().unwrap(), "bug");
        assert_eq!(labels[1].as_str().unwrap(), "points:3");
        assert_eq!(result.points, 3);
    }

    // ========================================================================
    // Test: PRs are ignored
    // ========================================================================

    #[tokio::test]
    async fn test_sync_ignores_pull_requests() {
        let issues = vec![
            make_api_issue(1, 1, "Real Issue", "open", vec![], vec![], false, None),
            make_api_issue(2, 2, "A PR", "open", vec![], vec![], true, None),
        ];

        let github_service: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues,
            repo_id: 100,
        });
        let issue_repo: Arc<dyn GithubIssueRepository> = Arc::new(FakeGithubIssueRepo {
            issues: std::sync::Mutex::new(vec![]),
        });

        let synced = sync_github_issues(
            github_service,
            issue_repo.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        assert_eq!(synced, 1);
        // Only the real issue should be persisted
        assert!(issue_repo.find_by_key(100, 1).await.unwrap().is_some());
        assert!(issue_repo.find_by_key(100, 2).await.unwrap().is_none());
    }

    // ========================================================================
    // Test: idempotent upsert (running sync twice yields no duplicates)
    // ========================================================================

    #[tokio::test]
    async fn test_sync_idempotent_upsert() {
        let issues = vec![make_api_issue(
            1,
            1,
            "Issue 1",
            "open",
            vec!["points:2"],
            vec!["bob"],
            false,
            None,
        )];

        let github_service: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues,
            repo_id: 200,
        });
        let issue_repo = Arc::new(FakeGithubIssueRepo {
            issues: std::sync::Mutex::new(vec![]),
        });
        let issue_repo_trait: Arc<dyn GithubIssueRepository> = issue_repo.clone();

        // Sync twice
        sync_github_issues(
            github_service.clone(),
            issue_repo_trait.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        sync_github_issues(
            github_service.clone(),
            issue_repo_trait.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        // Should still only have 1 issue (no duplicates)
        let count = issue_repo.issues.lock().unwrap().len();
        assert_eq!(count, 1);
    }

    // ========================================================================
    // Test: closed issue is reflected
    // ========================================================================

    #[tokio::test]
    async fn test_sync_closed_issue_reflected() {
        let issues = vec![make_api_issue(
            10,
            10,
            "Closed Issue",
            "closed",
            vec!["points:5"],
            vec!["charlie"],
            false,
            Some("2025-06-01T12:00:00Z"),
        )];

        let github_service: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues,
            repo_id: 300,
        });
        let issue_repo: Arc<dyn GithubIssueRepository> = Arc::new(FakeGithubIssueRepo {
            issues: std::sync::Mutex::new(vec![]),
        });

        sync_github_issues(
            github_service,
            issue_repo.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        let issue = issue_repo.find_by_key(300, 10).await.unwrap().unwrap();
        assert_eq!(issue.state, "closed");
        assert!(issue.closed_at.is_some());
        assert_eq!(issue.points, 5);
        // rewarded_sepolia should not be set during sync
        assert!(!issue.rewarded_sepolia);
    }

    // ========================================================================
    // Test: assignees are persisted
    // ========================================================================

    #[tokio::test]
    async fn test_sync_persists_assignees() {
        let issues = vec![make_api_issue(
            20,
            20,
            "Issue with assignees",
            "open",
            vec![],
            vec!["alice", "bob"],
            false,
            None,
        )];

        let github_service: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues,
            repo_id: 400,
        });
        let issue_repo: Arc<dyn GithubIssueRepository> = Arc::new(FakeGithubIssueRepo {
            issues: std::sync::Mutex::new(vec![]),
        });

        sync_github_issues(
            github_service,
            issue_repo.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        let issue = issue_repo.find_by_key(400, 20).await.unwrap().unwrap();
        let logins = issue.assignee_logins.as_array().unwrap();
        assert_eq!(logins.len(), 2);
        assert_eq!(logins[0].as_str().unwrap(), "alice");
        assert_eq!(logins[1].as_str().unwrap(), "bob");
    }

    // ========================================================================
    // Test: upsert updates changed fields (title change on re-sync)
    // ========================================================================

    #[tokio::test]
    async fn test_sync_upsert_updates_title() {
        let issue_repo = Arc::new(FakeGithubIssueRepo {
            issues: std::sync::Mutex::new(vec![]),
        });
        let issue_repo_trait: Arc<dyn GithubIssueRepository> = issue_repo.clone();

        // First sync with original title
        let issues_v1 = vec![make_api_issue(
            1,
            1,
            "Original Title",
            "open",
            vec![],
            vec![],
            false,
            None,
        )];
        let svc1: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues: issues_v1,
            repo_id: 500,
        });
        sync_github_issues(
            svc1,
            issue_repo_trait.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        // Second sync with updated title
        let issues_v2 = vec![make_api_issue(
            1,
            1,
            "Updated Title",
            "open",
            vec![],
            vec![],
            false,
            None,
        )];
        let svc2: Arc<dyn GithubService> = Arc::new(FakeGithubService {
            issues: issues_v2,
            repo_id: 500,
        });
        sync_github_issues(
            svc2,
            issue_repo_trait.clone(),
            vec!["org/repo".to_string()],
            None,
        )
        .await
        .unwrap();

        let count = issue_repo.issues.lock().unwrap().len();
        assert_eq!(count, 1);

        let issue = issue_repo_trait.find_by_key(500, 1).await.unwrap().unwrap();
        assert_eq!(issue.title, "Updated Title");
    }
}
