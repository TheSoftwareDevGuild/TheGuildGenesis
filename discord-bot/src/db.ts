import { Pool } from 'pg';
import { env } from './env.js';
import { logger } from './logger.js';

export const pool = new Pool({
  connectionString: env.DATABASE_URL,
  max: 10,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

export async function testConnection(): Promise<void> {
  try {
    const client = await pool.connect();
    await client.query('SELECT 1');
    client.release();
    logger.info('Database connection successful');
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Database connection failed');
    throw error;
  }
}

export interface ActivityEvent {
  id: string;
  user_id: string;
  user_name: string;
  amount: number;
  event_type: string;
  date: Date;
  guild_id?: string;
  processed_status: boolean;
  created_at: Date;
}

export async function insertActivityEvent(
  userId: string,
  userName: string,
  amount: number = env.POINTS_PER_MESSAGE,
  guildId?: string
): Promise<string> {
  const query = `
    INSERT INTO activity_events (user_id, user_name, amount, event_type, guild_id, processed_status)
    VALUES ($1, $2, $3, 'message', $4, FALSE)
    RETURNING id
  `;
  
  try {
    const result = await pool.query(query, [userId, userName, amount, guildId]);
    return result.rows[0].id;
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to insert activity event');
    throw error;
  }
}

export async function getUnprocessedEvents(): Promise<ActivityEvent[]> {
  const query = `
    SELECT id, user_id, user_name, amount, event_type, date, guild_id, processed_status, created_at
    FROM activity_events
    WHERE processed_status = FALSE
    ORDER BY created_at ASC
  `;
  
  try {
    const result = await pool.query(query);
    return result.rows;
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to get unprocessed events');
    throw error;
  }
}

export async function markEventAsProcessed(eventId: string): Promise<void> {
  const query = `
    UPDATE activity_events
    SET processed_status = TRUE
    WHERE id = $1
  `;
  
  try {
    await pool.query(query, [eventId]);
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to mark event as processed');
    throw error;
  }
}

export async function getUserBalance(guildId: string, userId: string): Promise<number> {
  const query = `
    SELECT COALESCE(SUM(amount), 0) as total
    FROM activity_events
    WHERE guild_id = $1 AND user_id = $2
  `;
  
  try {
    const result = await pool.query(query, [guildId, userId]);
    return parseInt(result.rows[0].total) || 0;
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to get user balance');
    throw error;
  }
}

export interface LeaderboardEntry {
  user_id: string;
  user_name: string;
  total_points: number;
}

export async function getLeaderboard(
  guildId: string,
  limit: number = 10
): Promise<LeaderboardEntry[]> {
  const query = `
    SELECT 
      user_id,
      MAX(user_name) as user_name,
      SUM(amount) as total_points
    FROM activity_events
    WHERE guild_id = $1
    GROUP BY user_id
    ORDER BY total_points DESC
    LIMIT $2
  `;
  
  try {
    const result = await pool.query(query, [guildId, limit]);
    return result.rows.map(row => ({
      user_id: row.user_id,
      user_name: row.user_name,
      total_points: parseInt(row.total_points),
    }));
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to get leaderboard');
    throw error;
  }
}

export async function closePool(): Promise<void> {
  await pool.end();
  logger.info('Database pool closed');
}