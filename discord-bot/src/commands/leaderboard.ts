import { SlashCommandBuilder, EmbedBuilder, ChatInputCommandInteraction } from 'discord.js';
import { getLeaderboard } from '../db.js';
import { commandRateLimiter } from '../utils/rateLimiter.js';
import { logger } from '../logger.js';

export const leaderboardCommand = {
  data: new SlashCommandBuilder()
    .setName('leaderboard')
    .setDescription('Shows top members by activity points')
    .addIntegerOption(option =>
      option
        .setName('limit')
        .setDescription('Number of users to display (5-25)')
        .setRequired(false)
        .setMinValue(5)
        .setMaxValue(25)
    ),

  async execute(interaction: ChatInputCommandInteraction): Promise<void> {
    if (!commandRateLimiter.checkLimit(interaction.user.id)) {
      await interaction.reply({
        content: 'You are sending too many requests. Please try again later.',
        ephemeral: true,
      });
      return;
    }

    const limit = interaction.options.getInteger('limit') || 10;
    const guildId = interaction.guildId;

    if (!guildId) {
      await interaction.reply({
        content: 'This command can only be used in a server.',
        ephemeral: true,
      });
      return;
    }

    try {
      const leaderboard = await getLeaderboard(guildId, limit);

      if (leaderboard.length === 0) {
        await interaction.reply({
          content: 'No activity yet for the selected timeframe.',
        });
        return;
      }

      const leaderboardWithNames = await Promise.all(
        leaderboard.map(async (entry, index) => {
          let displayName = entry.user_name || 'Unknown User';
          let isLeft = false;

          try {
            const member = await interaction.guild?.members.fetch(entry.user_id);
            if (member) {
              displayName = member.displayName;
            }
          } catch {
            isLeft = true;
          }

          if (displayName.length > 30) {
            displayName = displayName.substring(0, 27) + '...';
          }

          return {
            rank: index + 1,
            displayName: isLeft ? `${displayName} (left)` : displayName,
            points: entry.total_points,
          };
        })
      );

      const embed = new EmbedBuilder()
        .setTitle('🏆 Activity Leaderboard')
        .setColor(0x00AE86)
        .setDescription(
          leaderboardWithNames
            .map(entry => 
              `**${entry.rank}.** ${entry.displayName} - ${entry.points.toLocaleString()} points`
            )
            .join('\n')
        )
        .setFooter({ text: `Top ${limit} members` })
        .setTimestamp();

      await interaction.reply({
        embeds: [embed],
      });

      logger.info(`Leaderboard command executed by ${interaction.user.username} with limit ${limit}`);
    } catch (error) {
      logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Error executing leaderboard command');
      await interaction.reply({
        content: 'An error occurred while fetching the leaderboard.',
        ephemeral: true,
      });
    }
  },
};