ALTER TABLE profiles ADD COLUMN IF NOT EXISTS linkedin_account TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS unique_linkedin_account_lower ON profiles (LOWER(linkedin_account));
