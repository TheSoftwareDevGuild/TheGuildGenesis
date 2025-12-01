-- Adds a distributions table which stores each distributed badge item.
-- Each row is one delivered badge and belongs to a batch (distribution_id).

CREATE TABLE IF NOT EXISTS distributions (
  id UUID PRIMARY KEY,
  distribution_id UUID NOT NULL,
  address TEXT NOT NULL,
  badge_name TEXT NOT NULL,
  metadata JSONB,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_distributions_distribution_id ON distributions(distribution_id);

CREATE INDEX IF NOT EXISTS idx_distributions_address ON distributions(address);
