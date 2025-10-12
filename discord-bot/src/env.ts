import { z } from 'zod';

const envSchema = z.object({
  DISCORD_BOT_TOKEN: z.string().min(1, 'Discord bot token is required'),
  DISCORD_APP_ID: z.string().min(1, 'Discord app ID is required'),
  DISCORD_GUILD_ID: z.string().optional(),
  DATABASE_URL: z.string().url('Valid database URL is required'),
  POINTS_PER_MESSAGE: z.string().default('1').transform(Number),
  LOG_LEVEL: z.enum(['debug', 'info', 'warn', 'error']).default('info'),
  MAX_MESSAGES_PER_MINUTE: z.string().default('10').transform(Number),
});

export const env = envSchema.parse(process.env);

export type Env = z.infer<typeof envSchema>;
