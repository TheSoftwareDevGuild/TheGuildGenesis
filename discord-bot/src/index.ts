import { Client, GatewayIntentBits, Events } from 'discord.js';
import { env } from './env.js';
import { logger } from './logger.js';
import { testConnection, closePool } from './db.js';
import { onMessageCreate } from './listeners/messageCreate.js';

// Rate limiting storage (in-memory)
interface UserRateLimit {
  count: number;
  resetTime: number;
}

const userMessageCounts = new Map<string, UserRateLimit>();
const RATE_LIMIT_WINDOW = 60 * 1000; // 1 minute

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
});

// Bot ready event
client.once(Events.ClientReady, (readyClient) => {
  logger.info(`Bot ready as ${readyClient.user.tag}`);
  logger.info(`Bot is in ${readyClient.guilds.cache.size} guilds`);
  
  if (env.DISCORD_GUILD_ID) {
    const targetGuild = readyClient.guilds.cache.get(env.DISCORD_GUILD_ID);
    if (targetGuild) {
      logger.info(`Tracking messages in guild: ${targetGuild.name}`);
    } else {
      logger.warn(`Target guild ${env.DISCORD_GUILD_ID} not found`);
    }
  } else {
    logger.info('Tracking messages in all guilds');
  }
});

// Message create event
client.on(Events.MessageCreate, onMessageCreate);

// Error handling
client.on(Events.Error, (error) => {
  logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Discord client error');
});

client.on(Events.Warn, (warning) => {
  logger.warn({ warning: String(warning) }, 'Discord client warning');
});

// Graceful shutdown
async function shutdown() {
  logger.info('Shutting down bot...');
  
  try {
    await client.destroy();
    await closePool();
    logger.info('Bot shutdown complete');
    process.exit(0);
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Error during shutdown');
    process.exit(1);
  }
}

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);

// Handle uncaught exceptions
process.on('uncaughtException', (error) => {
  logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Uncaught exception');
  shutdown();
});

process.on('unhandledRejection', (reason, promise) => {
  logger.error({ reason: String(reason), promise: String(promise) }, 'Unhandled rejection');
  shutdown();
});

// Start the bot
async function start() {
  try {
    // Test database connection first
    await testConnection();
    
    // Login to Discord
    await client.login(env.DISCORD_BOT_TOKEN);
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to start bot');
    process.exit(1);
  }
}

start();
