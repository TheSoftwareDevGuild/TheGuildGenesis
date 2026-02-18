CREATE TABLE IF NOT EXISTS github_issues (
    repo_id BIGINT NOT NULL,
    github_issue_id BIGINT NOT NULL,
    repo TEXT NOT NULL,
    issue_number INT NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('open', 'closed')),
    labels JSONB NOT NULL DEFAULT '[]',
    points INT NOT NULL DEFAULT 0,
    assignee_logins JSONB NOT NULL DEFAULT '[]',
    url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    closed_at TIMESTAMPTZ,
    rewarded_sepolia BOOLEAN NOT NULL DEFAULT false,
    distribution_id TEXT,
    updated_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (repo_id, github_issue_id)
);

CREATE INDEX IF NOT EXISTS idx_github_issues_repo ON github_issues(repo);
CREATE INDEX IF NOT EXISTS idx_github_issues_state ON github_issues(state);
CREATE INDEX IF NOT EXISTS idx_github_issues_rewarded_sepolia ON github_issues(rewarded_sepolia);
