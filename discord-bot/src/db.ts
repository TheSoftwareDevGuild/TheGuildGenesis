import { Pool, PoolClient } from 'pg';
import { env } from './env.js';
import { logger } from './logger.js';

export const pool = new Pool({
  connectionString: env.DATABASE_URL,
  max: 10,
  idleTimeoutMillis: 30000,
  connectionTimeoutMillis: 2000,
});

// Test database connection on startup
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
  processed_status: boolean;
  created_at: Date;
}

export async function insertActivityEvent(
  userId: string,
  userName: string,
  amount: number = env.POINTS_PER_MESSAGE
): Promise<string> {
  const query = `
    INSERT INTO activity_events (user_id, user_name, amount, event_type, processed_status)
    VALUES ($1, $2, $3, 'message', FALSE)
    RETURNING id
  `;
  
  try {
    const result = await pool.query(query, [userId, userName, amount]);
    return result.rows[0].id;
  } catch (error) {
    logger.error({ error: error instanceof Error ? error.message : String(error) }, 'Failed to insert activity event');
    throw error;
  }
}

export async function getUnprocessedEvents(): Promise<ActivityEvent[]> {
  const query = `
    SELECT id, user_id, user_name, amount, event_type, date, processed_status, created_at
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

// Graceful shutdown
export async function closePool(): Promise<void> {
  await pool.end();
  logger.info('Database pool closed');
}
