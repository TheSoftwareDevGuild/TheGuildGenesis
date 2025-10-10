-- Add login_nonce column to profiles table
-- The nonce starts at 1 and increments with each successful login
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS login_nonce BIGINT NOT NULL DEFAULT 1;
