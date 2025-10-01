#!/usr/bin/env node

// Simple test script to verify the bot can connect to Discord
// This script will test the environment configuration and database connection

import { env } from './src/env.js';
import { testConnection, closePool } from './src/db.js';
import { logger } from './src/logger.js';

async function testBotSetup() {
  logger.info('Testing Discord bot setup...');
  
  // Test environment variables
  logger.info('Environment variables loaded:', {
    hasBotToken: !!env.DISCORD_BOT_TOKEN,
    hasAppId: !!env.DISCORD_APP_ID,
    hasGuildId: !!env.DISCORD_GUILD_ID,
    hasDatabaseUrl: !!env.DATABASE_URL,
    pointsPerMessage: env.POINTS_PER_MESSAGE,
    logLevel: env.LOG_LEVEL,
    maxMessagesPerMinute: env.MAX_MESSAGES_PER_MINUTE,
  });
  
  // Test database connection
  try {
    await testConnection();
    logger.info('✅ Database connection test passed');
  } catch (error) {
    logger.error('❌ Database connection test failed:', error);
    process.exit(1);
  }
  
  // Clean up
  await closePool();
  
  logger.info('✅ All tests passed! Bot is ready to run.');
  logger.info('To start the bot, run: npm run dev');
}

testBotSetup().catch((error) => {
  logger.error('Test failed:', error);
  process.exit(1);
});
