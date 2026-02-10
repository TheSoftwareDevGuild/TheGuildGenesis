ALTER TABLE projects
    ADD COLUMN IF NOT EXISTS owner_address VARCHAR(42)
        GENERATED ALWAYS AS (creator) STORED;

CREATE INDEX IF NOT EXISTS idx_projects_owner_address ON projects(owner_address);