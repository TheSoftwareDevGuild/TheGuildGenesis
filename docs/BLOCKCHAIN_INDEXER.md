# Blockchain Indexer Documentation

## Overview

The Guild Genesis blockchain indexer is a high-performance service that monitors and indexes on-chain events from TheGuildBadgeRegistry Activity token. It extracts blockchain data, transforms it into a structured format, and stores it in a PostgreSQL database for fast querying and retrieval by the backend API.

## Architecture

### Components

The indexer consists of several key components:

1. **Event Listener**: Monitors the blockchain for new events emitted by the smart contract
2. **Data Transformer**: Converts raw blockchain event data into structured database records
3. **Database Handler**: Manages storage and retrieval of indexed data in PostgreSQL
4. **Re-org Handler**: Manages blockchain reorganizations to ensure data consistency
5. **Configuration Manager**: Centralized configuration with validation
6. **API Server**: RESTful endpoints for querying indexed data

### How It Works

The indexer uses a domain-driven design with clean architecture principles:
1. Event Polling (Infrastructure Layer)

AlloyEthereumEventPollingService implements the polling logic using the Alloy library
Connects to the blockchain via RPC (https://reth-ethereum.ithaca.xyz/rpc)
Fetches the latest block number
Creates a filter for Transfer(from,to,value) events
Retrieves all logs matching the filter from the latest block

2. Event Processing (Domain Layer)

Decodes raw log data using ITokenActivity::Transfer::decode_log_data()
Filters for minting events (from == Address::ZERO)
Creates EthereumEvent domain entities with:

Transaction hash and log index (for unique ID)
Recipient address (to)
Token amount (value)
Event type (ActivityTokenMinted)



3. Data Persistence (Infrastructure Layer)

Stores processed events in PostgreSQL using SQLx
Ensures data integrity with primary key constraints
Indexes key fields for efficient querying

4. Continuous Operation

Polls at configured intervals (default: 12 seconds)
Handles errors with retry logic
Monitors for blockchain reorganizations

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
         │
         │ Query API
         │
         ▼
┌─────────────────┐
│  REST API       │◄──── Exposes data to clients
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
- `id`: The unique identifier for the badge
- `description`: Badge description
- `creator`: Ethereum address that created the badge
- `block_number`: Block number when the badge was created
- `transaction_hash`: Transaction hash of the badge creation
- `timestamp`: Unix timestamp of the block
- `log_index`: Position of the event in the transaction logs

Infrastructure Setup

## Infrastructure Setup

### Prerequisites

- **Rust**: v1.70 or higher
- **PostgreSQL**: v14 or higher
- **Ethereum RPC endpoint**: Access to an Ethereum node (Infura, Alchemy, or self-hosted)
- **Docker** (optional): For containerized deployment

### Environment Variables

Create a `.env` file in the indexer directory:

```env
# Database Configuration
DATABASE_URL=postgresql://postgres:password@localhost:5432/guild_genesis

# Blockchain Configuration
RPC_URL=https://mainnet.infura.io/v3/YOUR_INFURA_KEY
CHAIN_ID=1
CONTRACT_ADDRESS=0x... # Activity Token contract address
START_BLOCK=0 # Block number to start indexing from

# Indexer Configuration
POLL_INTERVAL=12000 # Polling interval in milliseconds (12 seconds for Ethereum)
BATCH_SIZE=1000 # Number of blocks to process in each batch
MAX_RETRIES=3 # Maximum retry attempts for failed requests
CONFIRMATION_BLOCKS=12 # Number of blocks to wait before considering finalized

# Server Configuration
PORT=3002
LOG_LEVEL=info # Options: trace, debug, info, warn, error

# Monitoring (Optional)
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
cargo build --release
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
# Using the migration binary
cargo run --bin migrate-indexer

# Or using cargo
cargo sqlx migrate run
```

### Step 4: Verify Configuration

The indexer will validate all configuration on startup. Check logs for any errors.

## Running the Indexer

### Development Mode

Run the indexer with hot-reload and verbose logging:

```bash
RUST_LOG=debug cargo run --bin guild-indexer
```

### Production Mode

Build and run the optimized version:

```bash
cargo build --release
./target/release/guild-indexer
```

### Docker Deployment

#### Build the Docker image:

```bash
docker build -t guild-indexer .
```

#### Run with Docker Compose:

```bash
# Start all services (indexer + PostgreSQL)
docker-compose up -d

# View logs
docker-compose logs -f indexer

# Stop services
docker-compose down
```

Example `docker-compose.yml`:

```yaml
version: '3.8'

services:
  indexer:
    build: 
      context: .
      dockerfile: Dockerfile
    restart: unless-stopped
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/guild_genesis
      - RPC_URL=${RPC_URL}
      - CONTRACT_ADDRESS=${CONTRACT_ADDRESS}
      - START_BLOCK=${START_BLOCK:-0}
      - CHAIN_ID=${CHAIN_ID:-1}
      - POLL_INTERVAL=${POLL_INTERVAL:-12000}
      - BATCH_SIZE=${BATCH_SIZE:-1000}
      - MAX_RETRIES=${MAX_RETRIES:-3}
      - CONFIRMATION_BLOCKS=${CONFIRMATION_BLOCKS:-12}
      - PORT=3002
      - RUST_LOG=info
    ports:
      - "3002:3002"
    depends_on:
      db:
        condition: service_healthy
    networks:
      - guild-network
    healthcheck:
      test: ["CMD", "/app/healthcheck.sh"]
      interval: 30s
      timeout: 3s
      start_period: 10s
      retries: 3

  db:
    image: postgres:14-alpine
    restart: unless-stopped
    environment:
      POSTGRES_DB: guild_genesis
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    networks:
      - guild-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 10s
      timeout: 5s
      retries: 5

volumes:
  postgres_data:

networks:
  guild-network:
    driver: bridge
```

## Monitoring & Maintenance

### Health Check

Check if the indexer is running and synced:

```bash
curl http://localhost:3002/health
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

# Docker Compose
docker-compose logs -f indexer

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
START_BLOCK=18000000 cargo run --bin guild-indexer

# Using environment variable
START_BLOCK=18000000 ./target/release/guild-indexer
```

### Handling Chain Reorganizations

The indexer automatically handles chain reorganizations (re-orgs):

1. Monitors for block hash changes at previously indexed heights
2. Rolls back affected data when a re-org is detected
3. Re-indexes the canonical chain from the re-org point

Manual re-org recovery (if implemented):

```bash
# This would be a custom command if implemented
cargo run --bin guild-indexer -- --reorg-recovery --from-block=18456700
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

**Issue: Configuration validation errors**
- **Cause**: Invalid or missing environment variables
- **Solution**: 
  - Check `.env` file against `.env.example`
  - Ensure all required variables are set
  - Verify CONTRACT_ADDRESS format (0x + 40 hex chars)

**Issue: Duplicate events**
- **Cause**: Re-org handling or restart during processing
- **Solution**: Database primary key constraints prevent duplicates automatically; check logs for warnings

**Issue: Missing events**
- **Cause**: RPC endpoint issues or gaps in block processing
- **Solution**: 
  - Verify RPC endpoint is reliable
  - Check for gaps in indexed blocks
  - Backfill missing ranges with START_BLOCK

**Issue: Database connection errors**
- **Cause**: PostgreSQL not accessible or connection pool exhausted
- **Solution**:
  - Verify `DATABASE_URL` is correct
  - Check PostgreSQL is running: `pg_isready -h localhost -p 5432`
  - Check connection pool settings

**Issue: Container health check failures**
- **Cause**: Service not fully started or port not accessible
- **Solution**:
  - Check container logs: `docker logs guild-indexer`
  - Verify PORT environment variable matches exposed port
  - Increase `start_period` in health check configuration

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug cargo run --bin guild-indexer
```

Or with specific modules:

```bash
RUST_LOG=guild_indexer=debug,sqlx=info cargo run --bin guild-indexer
```

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

## API Endpoints

The indexer exposes the following REST endpoints:

### Health Check
```
GET /health
```
Returns service health status and indexing statistics.

### Query Events
```
GET /api/events?type=BadgeCreated&limit=100&offset=0
```
Query indexed events with optional filters.

**Query Parameters:**
- `type`: Filter by event type
- `limit`: Maximum number of results (default: 100)
- `offset`: Pagination offset (default: 0)
- `from`: Start timestamp (ISO 8601)
- `to`: End timestamp (ISO 8601)

### Get Event by ID
```
GET /api/events/:id
```
Retrieve a specific event by its unique identifier.

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
- Optimize PostgreSQL configuration:
  ```sql
  -- postgresql.conf
  shared_buffers = 256MB
  work_mem = 8MB
  maintenance_work_mem = 128MB
  effective_cache_size = 1GB
  ```

**Horizontal Scaling:**
- Run multiple indexer instances for different block ranges
- Use read replicas for query load
- Implement event streaming (Kafka) for real-time consumers
- Deploy indexers per contract or event type

## Configuration Reference

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://user:pass@localhost:5432/db` |
| `RPC_URL` | Ethereum RPC endpoint | `https://mainnet.infura.io/v3/KEY` |
| `CONTRACT_ADDRESS` | Smart contract address | `0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb1` |

### Optional Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `CHAIN_ID` | `1` | Ethereum chain ID |
| `START_BLOCK` | `0` | Block to start indexing from |
| `POLL_INTERVAL` | `12000` | Polling interval in milliseconds |
| `BATCH_SIZE` | `1000` | Blocks per batch |
| `MAX_RETRIES` | `3` | Max retry attempts |
| `CONFIRMATION_BLOCKS` | `12` | Blocks before finality |
| `PORT` | `3002` | API server port |
| `LOG_LEVEL` | `info` | Logging level |

## Testing

### Running Tests

```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration_test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_database_connection
```

### Test Database Setup

Tests use a separate test database:

```bash
# Create test database
createdb guild_genesis_test

# Run migrations
DATABASE_URL=postgresql://postgres:password@localhost:5432/guild_genesis_test \
  cargo run --bin migrate-indexer
```

## Security Considerations

1. **RPC Endpoint Security**: 
   - Use authenticated endpoints
   - Don't expose API keys in logs or version control
   - Rotate keys regularly

2. **Database Access**: 
   - Restrict database access to indexer service only
   - Use read-only credentials for query endpoints
   - Enable SSL/TLS for database connections

3. **Rate Limiting**: 
   - Implement rate limiting to prevent RPC abuse
   - Use backoff strategies for retries
   - Monitor RPC usage

4. **Data Validation**: 
   - Validate all blockchain data before storage
   - Sanitize inputs for SQL queries (use parameterized queries)
   - Verify event signatures

5. **Error Handling**: 
   - Never expose internal errors to external consumers
   - Log errors securely without sensitive data
   - Implement proper error boundaries

6. **Container Security**:
   - Run as non-root user (already configured)
   - Use minimal base images
   - Scan images for vulnerabilities
   - Keep dependencies updated

## Maintenance Schedule

**Daily:**
- Monitor sync status and lag
- Check error logs for anomalies
- Verify disk space availability

**Weekly:**
- Review performance metrics
- Check for missed events
- Update RPC endpoints if needed
- Review security logs

**Monthly:**
- Database maintenance (VACUUM, ANALYZE)
- Review and optimize slow queries
- Update dependencies (security patches)
- Review and rotate credentials
- Backup database

**Quarterly:**
- Performance audit
- Capacity planning
- Disaster recovery testing
- Security audit

## Continuous Integration

The project includes a GitHub Actions workflow for automated testing:

```yaml
# .github/workflows/rust.yml
name: Rust CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main, develop ]

# See rust.yml file for complete configuration
```

**CI Pipeline includes:**
- Code formatting checks (rustfmt)
- Linting (clippy)
- Unit tests
- Integration tests
- Release builds

## Support & Resources

- **Documentation**: `/docs/`
- **GitHub Issues**: [TheGuildGenesis/issues](https://github.com/TheSoftwareDevGuild/TheGuildGenesis/issues)
- **Discord**: [The Software Dev Guild Discord](https://discord.gg/pg4UgaTr)

## Contributing

To contribute to the indexer:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Run formatting (`cargo fmt`)
7. Run linting (`cargo clippy`)
8. Commit your changes (`git commit -m 'Add amazing feature'`)
9. Push to the branch (`git push origin feature/amazing-feature`)
10. Submit a pull request

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.

## Changelog

### Version 0.1.0 (Current)
- Initial release
- Activitytoken indexing
- PostgreSQL storage
- RESTful API
- Docker support
- Configuration validation
- Health monitoring
- Automatic re-org handling

## License

See [LICENSE](../LICENSE) file for details.

---