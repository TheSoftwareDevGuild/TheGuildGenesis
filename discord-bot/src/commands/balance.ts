import { SlashCommandBuilder, ChatInputCommandInteraction } from 'discord.js';
import { getUserBalance } from '../db.js';
import { commandRateLimiter } from '../utils/rateLimiter.js';
import { logger } from '../logger.js';

export const balanceCommand = {
  data: new SlashCommandBuilder()
    .setName('balance')
    .setDescription('Shows activity point balance')
    .addUserOption(option =>
      option
        .setName('user')
        .setDescription('User to check balance for (optional)')
        .setRequired(false)
    ),

  async execute(interaction: ChatInputCommandInteraction): Promise<void> {
    if (!commandRateLimiter.checkLimit(interaction.user.id)) {
      await interaction.reply({
        content: 'You are sending too many requests. Please try again later.',
        ephemeral: true,
      });
      return;
    }

    const targetUser = interaction.options.getUser('user') || interaction.user;
    const guildId = interaction.guildId;

    if (!guildId) {
      await interaction.reply({
        content: 'This command can only be used in a server.',
        ephemeral: true,
      });
      return;
    }

    try {
      const member = await interaction.guild?.members.fetch(targetUser.id).catch(() => null);
      
      if (!member) {
        await interaction.reply({
          content: 'User not found in this guild.',
          ephemeral: true,
        });
        return;
      }

      const balance = await getUserBalance(guildId, targetUser.id);
      const displayName = member.displayName;

      await interaction.reply({
        content: `${displayName} has ${balance.toLocaleString()} activity points.`,
        ephemeral: true,
      });

      logger.info(`Balance command executed by ${interaction.user.username} for user ${targetUser.username}`);
    } catch (error) {
      logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Error executing balance command');
      await interaction.reply({
        content: 'An error occurred while fetching the balance.',
        ephemeral: true,
      });
    }
  },
};