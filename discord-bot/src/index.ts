import { Client, GatewayIntentBits, Events } from 'discord.js';
import { env } from './env.js';
import { logger } from './logger.js';
import { testConnection, closePool } from './db.js';
import { onMessageCreate } from './listeners/messageCreate.js';
import { commands, registerCommands } from './commands/index.js';

const client = new Client({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.MessageContent,
  ],
});

client.once(Events.ClientReady, async (readyClient) => {
  logger.info(`Bot ready as ${readyClient.user.tag}`);
  logger.info(`Bot is in ${readyClient.guilds.cache.size} guilds`);
  
  try {
    await registerCommands();
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to register commands');
  }
  
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

client.on(Events.MessageCreate, onMessageCreate);

client.on(Events.InteractionCreate, async (interaction) => {
  if (!interaction.isChatInputCommand()) return;

  const command = commands.get(interaction.commandName);

  if (!command) {
    logger.warn(`Unknown command: ${interaction.commandName}`);
    return;
  }

  try {
    await command.execute(interaction);
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, `Error executing command: ${interaction.commandName}`);
    
    const errorMessage = {
      content: 'There was an error while executing this command!',
      ephemeral: true,
    };
    
    if (interaction.replied || interaction.deferred) {
      await interaction.followUp(errorMessage);
    } else {
      await interaction.reply(errorMessage);
    }
  }
});

client.on(Events.Error, (error) => {
  logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Discord client error');
});

client.on(Events.Warn, (warning) => {
  logger.warn({ warning: String(warning) }, 'Discord client warning');
});

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

process.on('uncaughtException', (error) => {
  logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Uncaught exception');
  shutdown();
});

process.on('unhandledRejection', (reason, promise) => {
  logger.error({ reason: String(reason), promise: String(promise) }, 'Unhandled rejection');
  shutdown();
});

async function start() {
  try {
    await testConnection();
    await client.login(env.DISCORD_BOT_TOKEN);
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to start bot');
    process.exit(1);
  }
}

start();