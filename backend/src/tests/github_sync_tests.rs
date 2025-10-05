#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use serde_json::json;

    use crate::application::commands::github_sync::{
        derive_points, transform_issue,
    };
    use crate::domain::services::github_api_service::{
        GithubAssignee, GithubIssueApi, GithubLabel, GithubRepoApi,
    };

    #[test]
    fn test_derive_points_from_labels() {
        let labels = vec![GithubLabel { name: "points:5".into() }];
        assert_eq!(derive_points(&labels), 5);
        let labels = vec![GithubLabel { name: "Points:2".into() }];
        assert_eq!(derive_points(&labels), 2);
        let labels = vec![GithubLabel { name: "other".into() }];
        assert_eq!(derive_points(&labels), 0);
    }

    #[test]
    fn test_transform_issue_ignores_prs_and_normalizes_labels() {
        let repo = GithubRepoApi { id: 123, full_name: "org/repo".into() };
        let ia = GithubIssueApi {
            id: 999,
            number: 1,
            title: "Issue".into(),
            state: "open".into(),
            html_url: "https://example".into(),
            pull_request: None,
            labels: vec![GithubLabel { name: "Points:3".into() }, GithubLabel { name: "Bug".into() }],
            assignees: vec![GithubAssignee { login: "alice".into() }],
            created_at: Utc.timestamp_opt(1_700_000_000, 0).unwrap(),
            closed_at: None,
            updated_at: Utc.timestamp_opt(1_700_000_100, 0).unwrap(),
        };
        let issue = transform_issue(&repo, &ia).unwrap();
        assert_eq!(issue.points, 3);
        assert_eq!(issue.state, "open");
        assert_eq!(issue.labels, json!(["points:3", "bug"]));
        assert_eq!(issue.assignee_logins, json!(["alice"]));

        // With pull_request set, should be ignored
        let mut ia_pr = ia.clone();
        ia_pr.pull_request = Some(json!({"url": "..."}));
        assert!(transform_issue(&repo, &ia_pr).is_none());
    }
}