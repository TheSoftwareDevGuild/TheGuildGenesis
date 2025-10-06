ALTER TABLE activity_events 
ADD COLUMN IF NOT EXISTS guild_id TEXT;

CREATE INDEX IF NOT EXISTS idx_activity_events_guild_id ON activity_events(guild_id);
CREATE INDEX IF NOT EXISTS idx_activity_events_guild_user ON activity_events(guild_id, user_id);

COMMENT ON COLUMN activity_events.guild_id IS 'Discord guild (server) ID where the activity occurred';