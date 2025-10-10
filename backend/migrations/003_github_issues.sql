-- GitHub Issues ingestion table
CREATE TABLE IF NOT EXISTS github_issues (
    repo TEXT NOT NULL,
    repo_id BIGINT NOT NULL,
    github_issue_id BIGINT NOT NULL,
    number INT NOT NULL,
    title TEXT NOT NULL,
    state TEXT NOT NULL CHECK (state IN ('open','closed')),
    labels JSONB NOT NULL DEFAULT '[]'::jsonb,
    points INT NOT NULL DEFAULT 0,
    assignee_logins JSONB NOT NULL DEFAULT '[]'::jsonb,
    html_url TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    closed_at TIMESTAMP WITH TIME ZONE NULL,
    rewarded BOOL NOT NULL DEFAULT FALSE,
    distribution_id TEXT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (repo_id, github_issue_id)
);

CREATE INDEX IF NOT EXISTS idx_github_issues_repo ON github_issues(repo);
CREATE INDEX IF NOT EXISTS idx_github_issues_state ON github_issues(state);
CREATE INDEX IF NOT EXISTS idx_github_issues_number ON github_issues(number);