# Discord Bot

A Discord bot that tracks user activity and awards points for messages sent in Discord servers.

## Features

- Tracks user messages and awards activity points
- Stores activity events in PostgreSQL database
- Rate limiting to prevent spam
- Configurable guild filtering
- Structured logging with Pino

## Quick Start

### 1. Environment Setup

Create a `.env` file in the `discord-bot` directory:

```bash
# Discord Bot Configuration
DISCORD_BOT_TOKEN=your_bot_token_here
DISCORD_APP_ID=your_app_id_here
DISCORD_GUILD_ID=your_guild_id_here

# Database Configuration
DATABASE_URL=postgresql://guild_user:guild_password@localhost:5433/guild_genesis

# Bot Behavior
POINTS_PER_MESSAGE=1
LOG_LEVEL=info
MAX_MESSAGES_PER_MINUTE=10
```

### 2. Database Migration

Start PostgreSQL and run the migration:

```bash
# Start PostgreSQL (if using Docker)
docker-compose up postgres -d

# Run migration
psql postgresql://guild_user:guild_password@localhost:5433/guild_genesis -f migrations/001_create_activity_events.sql
```

### 3. Run the Bot

```bash
# Development mode
npm run dev

# Production mode
npm run build && npm start
```

### 4. Docker Deployment

```bash
# Build and run with docker-compose
docker-compose up discord-bot
```

## Database Schema

The bot creates an `activity_events` table with the following structure:

- `id`: UUID primary key
- `user_id`: Discord user ID (stable identifier)
- `user_name`: Discord username (may change)
- `amount`: Points awarded (default: 1)
- `event_type`: Type of event (currently only 'message')
- `date`: Timestamp of the event
- `processed_status`: Whether the event has been processed for minting
- `created_at`: Record creation timestamp

## Architecture

- **src/index.ts**: Main bot logic and Discord.js client
- **src/env.ts**: Environment variable validation with Zod
- **src/db.ts**: Database connection and activity event operations
- **src/logger.ts**: Structured logging configuration
- **migrations/**: SQL migration files

## Rate Limiting

The bot implements basic rate limiting to prevent spam:
- Maximum 10 messages per user per minute
- In-memory tracking (resets on bot restart)

## Future Enhancements

- Additional event types (votes, reactions, etc.)
- Persistent rate limiting with Redis
- Health check endpoint
- Metrics and monitoring
