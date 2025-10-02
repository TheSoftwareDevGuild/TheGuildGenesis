import { Message } from 'discord.js';
import { env } from '../env.js';
import { logger } from '../logger.js';
import { insertActivityEvent } from '../db.js';

// Rate limiting storage (in-memory)
interface UserRateLimit {
  count: number;
  resetTime: number;
}
const userMessageCounts = new Map<string, UserRateLimit>();
const RATE_LIMIT_WINDOW = 60 * 1000; // 1 minute

export async function onMessageCreate(message: Message): Promise<void> {
  // Skip bots
  if (message.author.bot) return;

  // Skip if guild filter is set and doesn't match
  if (env.DISCORD_GUILD_ID && message.guild?.id !== env.DISCORD_GUILD_ID) {
    return;
  }

  // Rate limiting check
  const userId = message.author.id;
  const now = Date.now();
  const userLimit = userMessageCounts.get(userId);

  if (userLimit && now < userLimit.resetTime) {
    if (userLimit.count >= env.MAX_MESSAGES_PER_MINUTE) {
      logger.debug(`Rate limited user ${message.author.username} (${userId})`);
      return;
    }
    userLimit.count++;
  } else {
    userMessageCounts.set(userId, {
      count: 1,
      resetTime: now + RATE_LIMIT_WINDOW,
    });
  }

  try {
    const eventId = await insertActivityEvent(
      message.author.id,
      message.author.username,
      env.POINTS_PER_MESSAGE
    );

    logger.info(
      `Activity event created: ${eventId} for user ${message.author.username} (${userId})`
    );
  } catch (error) {
    logger.error(
      { error: error instanceof Error ? error.message : String(error) },
      'Failed to insert activity event'
    );
  }
}

