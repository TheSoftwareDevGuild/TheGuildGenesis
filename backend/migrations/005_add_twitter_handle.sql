ALTER TABLE profiles ADD COLUMN IF NOT EXISTS twitter_handle TEXT;
CREATE UNIQUE INDEX IF NOT EXISTS unique_twitter_handle_lower ON profiles (LOWER(twitter_handle));
