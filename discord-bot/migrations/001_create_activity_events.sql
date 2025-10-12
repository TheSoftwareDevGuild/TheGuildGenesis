-- Create activity_events table for Discord bot
-- This table stores activity points awarded to users for various events

CREATE TABLE IF NOT EXISTS activity_events (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id TEXT NOT NULL,
  user_name TEXT NOT NULL,
  amount INTEGER NOT NULL DEFAULT 1,
  event_type TEXT NOT NULL CHECK (event_type IN ('message')),
  date TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  processed_status BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_activity_events_user_id ON activity_events(user_id);
CREATE INDEX IF NOT EXISTS idx_activity_events_processed ON activity_events(processed_status);
CREATE INDEX IF NOT EXISTS idx_activity_events_date ON activity_events(date);
CREATE INDEX IF NOT EXISTS idx_activity_events_event_type ON activity_events(event_type);

-- Add comment for documentation
COMMENT ON TABLE activity_events IS 'Stores activity points awarded to Discord users for various events';
COMMENT ON COLUMN activity_events.user_id IS 'Discord user ID (stable identifier)';
COMMENT ON COLUMN activity_events.user_name IS 'Discord username (may change over time)';
COMMENT ON COLUMN activity_events.amount IS 'Number of points awarded for this event';
COMMENT ON COLUMN activity_events.event_type IS 'Type of event that triggered the points (message, vote, etc.)';
COMMENT ON COLUMN activity_events.processed_status IS 'Whether this event has been processed for token minting';
