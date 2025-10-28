# Blockchain Indexer Documentation

## Overview

The Guild Genesis blockchain indexer is a service that monitors and indexes on-chain events from TheGuildBadgeRegistry smart contract. It extracts blockchain data, transforms it into a structured format, and stores it in a PostgreSQL database for fast querying and retrieval by the backend API.

## Architecture

### Components

The indexer consists of several key components:

1. **Event Listener**: Monitors the blockchain for new events emitted by the smart contract
2. **Data Transformer**: Converts raw blockchain event data into structured database records
3. **Database Handler**: Manages storage and retrieval of indexed data in PostgreSQL
4. **Re-org Handler**: Manages blockchain reorganizations to ensure data consistency

### How It Works

```
┌─────────────────┐
│  Blockchain     │
│  (Ethereum/L2)  │
└────────┬────────┘
         │
         │ RPC Connection
         │
         ▼
┌─────────────────┐
│  Event Listener │◄──── Monitors BadgeCreated events
└────────┬────────┘
         │
         │ Raw Event Data
         │
         ▼
┌─────────────────┐
│  Transformer    │◄──── Decodes & structures data
└────────┬────────┘
         │
         │ Structured Records
         │
         ▼
┌─────────────────┐
│  PostgreSQL     │◄──── Stores indexed data
└─────────────────┘
```

## Indexed Events

### BadgeCreated Event

The primary event indexed by the service:

```solidity
event BadgeCreated(
    bytes32 indexed name,
    bytes32 description,
    address indexed creator
)
```

**Indexed Fields:**
- `id`: Unique identifier for the event (VARCHAR(255), PRIMARY KEY)
- `event_type`: Type of the event (TEXT, e.g., "BadgeCreated")
- `timestamp`: Timestamp when the event was indexed (TIMESTAMPTZ)
- `created_at`: Timestamp when the record was created (TIMESTAMPTZ)

## Infrastructure Setup

### Prerequisites

- **Node.js**: v18 or higher
- **PostgreSQL**: v14 or higher
- **Ethereum RPC endpoint**: Access to an Ethereum node (Infura, Alchemy, or self-hosted)
- **Environment variables configured**

### Environment Variables

Create a `.env` file in the indexer directory:

```env
# Database Configuration
DATABASE_URL=postgresql://user:password@localhost:5432/guild_genesis

# Blockchain Configuration
RPC_URL=https://mainnet.infura.io/v3/YOUR_INFURA_KEY
CHAIN_ID=1
CONTRACT_ADDRESS=0x... # TheGuildBadgeRegistry contract address
START_BLOCK=0 # Block number to start indexing from

# Indexer Configuration
POLL_INTERVAL=12000 # Polling interval in milliseconds (12 seconds for Ethereum)
BATCH_SIZE=1000 # Number of blocks to process in each batch
MAX_RETRIES=3 # Maximum retry attempts for failed requests
CONFIRMATION_BLOCKS=12 # Number of blocks to wait before considering finalized

# Monitoring (Optional)
LOG_LEVEL=info
SENTRY_DSN=... # Error tracking
METRICS_PORT=9090 # Prometheus metrics endpoint
```

### Database Schema

The indexer uses the following database table structure:

```sql
-- Ethereum events table
CREATE TABLE ethereum_events (
    id VARCHAR(255) PRIMARY KEY,
    event_type TEXT NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for efficient queries
CREATE INDEX idx_event_type ON ethereum_events(event_type);
CREATE INDEX idx_timestamp ON ethereum_events(timestamp);
CREATE INDEX idx_created_at ON ethereum_events(created_at);
```

**Column Descriptions:**

- **id** (VARCHAR(255)): Unique identifier for each event, typically constructed from transaction hash and log index
- **event_type** (TEXT): The type of event (e.g., "BadgeCreated", "BadgeUpdated")
- **timestamp** (TIMESTAMPTZ): The timestamp when the blockchain event occurred
- **created_at** (TIMESTAMPTZ): The timestamp when the record was inserted into the database

## Installation & Setup

### Step 1: Install Dependencies

```bash
cd indexer
npm install
```

### Step 2: Configure Environment

Copy the example environment file and update with your values:

```bash
cp .env.example .env
# Edit .env with your configuration
```

### Step 3: Initialize Database

Run the database migrations:

```bash
npm run db:migrate
```

### Step 4: Verify Configuration

Test your RPC connection and contract configuration:

```bash
npm run verify-config
```

## Running the Indexer

### Development Mode

Run the indexer with hot-reload and verbose logging:

```bash
npm run dev
```

### Production Mode

Build and run the optimized version:

```bash
npm run build
npm start
```

### Docker Deployment

Build the Docker image:

```bash
docker build -t guild-indexer .
```

Run with Docker Compose:

```bash
docker-compose up -d indexer
```

Example `docker-compose.yml`:

```yaml
version: '3.8'

services:
  indexer:
    build: ./indexer
    restart: unless-stopped
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/guild_genesis
      - RPC_URL=${RPC_URL}
      - CONTRACT_ADDRESS=${CONTRACT_ADDRESS}
      - START_BLOCK=${START_BLOCK}
    depends_on:
      - db
    networks:
      - guild-network

  db:
    image: postgres:14
    environment:
      POSTGRES_DB: guild_genesis
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - guild-network

volumes:
  postgres_data:

networks:
  guild-network:
```

## Monitoring & Maintenance

### Health Check

Check if the indexer is running and synced:

```bash
curl http://localhost:9090/health
```

Response:
```json
{
  "status": "healthy",
  "lastIndexedBlock": 18456789,
  "currentBlock": 18456800,
  "blocksBehind": 11,
  "uptime": 86400
}
```

### Logs

View real-time logs:

```bash
# Docker
docker logs -f guild-indexer

# PM2
pm2 logs indexer

# Direct
tail -f logs/indexer.log
```

### Metrics

The indexer exposes Prometheus metrics at `http://localhost:9090/metrics`:

- `indexer_last_indexed_block`: Latest block number indexed
- `indexer_blocks_behind`: Number of blocks behind the chain tip
- `indexer_events_indexed_total`: Total number of events indexed
- `indexer_processing_duration_seconds`: Time taken to process blocks
- `indexer_errors_total`: Total number of errors encountered

### Backfilling

To reindex from a specific block:

```bash
# Set START_BLOCK in .env or pass as argument
npm run backfill -- --from-block=18000000 --to-block=18100000
```

### Handling Chain Reorganizations

The indexer automatically handles chain reorganizations (re-orgs):

1. Monitors for block hash changes at previously indexed heights
2. Rolls back affected data when a re-org is detected
3. Re-indexes the canonical chain from the re-org point

Manual re-org recovery:

```bash
npm run reorg-recovery -- --from-block=18456700
```

## Troubleshooting

### Common Issues

**Issue: Indexer falls behind**
- **Cause**: RPC rate limiting or slow database writes
- **Solution**: 
  - Increase `POLL_INTERVAL`
  - Reduce `BATCH_SIZE`
  - Optimize database indexes
  - Use a dedicated RPC endpoint

**Issue: Duplicate events**
- **Cause**: Re-org handling or restart during processing
- **Solution**: Database primary key constraints prevent duplicates automatically; check logs for warnings

**Issue: Missing events**
- **Cause**: RPC endpoint issues or gaps in block processing
- **Solution**: 
  - Run integrity check: `npm run verify-events`
  - Backfill missing ranges

**Issue: Database connection errors**
- **Cause**: PostgreSQL not accessible or connection pool exhausted
- **Solution**:
  - Verify `DATABASE_URL`
  - Check PostgreSQL is running
  - Increase connection pool size in configuration

### Debug Mode

Enable debug logging:

```bash
LOG_LEVEL=debug npm run dev
```

### Data Integrity Verification

Run the verification script to check for gaps:

```bash
npm run verify-integrity
```

This checks:
- Continuous block range (no gaps)
- Event consistency with blockchain
- Database constraint violations

## API Integration

The backend API can query indexed data:

```typescript
// Example: Get all events of a specific type
const events = await db.query(
  'SELECT * FROM ethereum_events WHERE event_type = $1 ORDER BY timestamp DESC',
  ['BadgeCreated']
);

// Example: Get recent events
const recentEvents = await db.query(
  'SELECT * FROM ethereum_events ORDER BY timestamp DESC LIMIT 10'
);

// Example: Get events within a time range
const timeRangeEvents = await db.query(
  `SELECT * FROM ethereum_events 
   WHERE timestamp BETWEEN $1 AND $2 
   ORDER BY timestamp ASC`,
  [startTime, endTime]
);

// Example: Count events by type
const eventCounts = await db.query(
  `SELECT event_type, COUNT(*) as count 
   FROM ethereum_events 
   GROUP BY event_type`
);
```

## Performance Optimization

### Recommended Settings

For optimal performance:

- **Batch Processing**: Process 500-1000 blocks per batch
- **Connection Pooling**: Set PostgreSQL max connections to 20-50
- **Caching**: Cache recent blocks to reduce RPC calls
- **Parallel Processing**: Process multiple block ranges in parallel for backfilling
- **Partitioning**: For high-volume chains, partition tables by timestamp ranges

### Scaling Considerations

**Vertical Scaling:**
- Increase server memory for larger batches
- Use SSD storage for PostgreSQL
- Optimize PostgreSQL configuration (shared_buffers, work_mem)

**Horizontal Scaling:**
- Run multiple indexer instances for different block ranges
- Use read replicas for query load
- Implement event streaming (Kafka) for real-time consumers

## Security Considerations

1. **RPC Endpoint Security**: Use authenticated endpoints; don't expose API keys
2. **Database Access**: Restrict database access to indexer service only
3. **Rate Limiting**: Implement rate limiting to prevent RPC abuse
4. **Data Validation**: Validate all blockchain data before storage
5. **Error Handling**: Never expose internal errors to external consumers

## Maintenance Schedule

**Daily:**
- Monitor sync status
- Check error logs
- Verify disk space

**Weekly:**
- Review performance metrics
- Check for missed events
- Update RPC endpoints if needed

**Monthly:**
- Database maintenance (VACUUM, ANALYZE)
- Review and optimize slow queries
- Update dependencies

## Support & Resources

- **Documentation**: `/docs/`
- **GitHub Issues**: [TheGuildGenesis/issues](https://github.com/TheSoftwareDevGuild/TheGuildGenesis/issues)
- **Discord**: [The Software Dev Guild Discord](https://discord.gg/pg4UgaTr)

## Contributing

To contribute to the indexer:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTION.md) for detailed guidelines.

## License

See [LICENSE](../LICENSE) file for details.