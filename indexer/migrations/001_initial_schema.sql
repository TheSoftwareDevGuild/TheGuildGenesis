-- Create the indexer schema
CREATE SCHEMA IF NOT EXISTS indexer;

-- Table for storing indexed Ethereum logs
CREATE TABLE IF NOT EXISTS indexer.ethereum_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    block_number BIGINT NOT NULL,
    block_hash VARCHAR(66) NOT NULL,
    transaction_hash VARCHAR(66) NOT NULL,
    transaction_index INTEGER NOT NULL,
    log_index INTEGER NOT NULL,
    address VARCHAR(42) NOT NULL,
    data TEXT NOT NULL,
    topics TEXT[] NOT NULL,
    removed BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for efficient querying by block number
CREATE INDEX IF NOT EXISTS idx_ethereum_logs_block_number ON indexer.ethereum_logs(block_number);

-- Index for efficient querying by transaction hash
CREATE INDEX IF NOT EXISTS idx_ethereum_logs_tx_hash ON indexer.ethereum_logs(transaction_hash);

-- Index for efficient querying by address
CREATE INDEX IF NOT EXISTS idx_ethereum_logs_address ON indexer.ethereum_logs(address);

-- Index for efficient querying by topics
CREATE INDEX IF NOT EXISTS idx_ethereum_logs_topics ON indexer.ethereum_logs USING GIN(topics);

-- Table for tracking indexing progress
CREATE TABLE IF NOT EXISTS indexer.indexing_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    chain_id INTEGER NOT NULL,
    last_indexed_block BIGINT NOT NULL,
    rpc_url TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, paused, error
    error_message TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Unique constraint to ensure one active indexer per chain
CREATE UNIQUE INDEX IF NOT EXISTS idx_indexing_progress_chain_id ON indexer.indexing_progress(chain_id) 
WHERE status = 'active';

-- Grant permissions to the guild_user
GRANT ALL PRIVILEGES ON SCHEMA indexer TO guild_user;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA indexer TO guild_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA indexer TO guild_user;
