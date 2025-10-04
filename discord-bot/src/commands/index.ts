import { Collection, REST, Routes, ChatInputCommandInteraction } from 'discord.js';
import { balanceCommand } from './balance.js';
import { leaderboardCommand } from './leaderboard.js';
import { env } from '../env.js';
import { logger } from '../logger.js';

export interface Command {
  data: {
    name: string;
    toJSON: () => any;
  };
  execute: (interaction: ChatInputCommandInteraction) => Promise<void>;
}

export const commands = new Collection<string, Command>();
commands.set(balanceCommand.data.name, balanceCommand);
commands.set(leaderboardCommand.data.name, leaderboardCommand);

export async function registerCommands(): Promise<void> {
  const rest = new REST().setToken(env.DISCORD_BOT_TOKEN);

  try {
    logger.info('Started refreshing application (/) commands.');

    const commandsData = Array.from(commands.values()).map(cmd => cmd.data.toJSON());

    if (env.DISCORD_GUILD_ID) {
      await rest.put(
        Routes.applicationGuildCommands(env.DISCORD_APP_ID, env.DISCORD_GUILD_ID),
        { body: commandsData }
      );
      logger.info(`Successfully registered ${commandsData.length} guild commands.`);
    } else {
      await rest.put(
        Routes.applicationCommands(env.DISCORD_APP_ID),
        { body: commandsData }
      );
      logger.info(`Successfully registered ${commandsData.length} global commands.`);
    }
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to register commands');
    throw error;
  }
}