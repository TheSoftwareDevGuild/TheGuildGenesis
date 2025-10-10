ALTER TABLE profiles ADD COLUMN IF NOT EXISTS github_login TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS unique_github_login_lower ON profiles (LOWER(github_login));
