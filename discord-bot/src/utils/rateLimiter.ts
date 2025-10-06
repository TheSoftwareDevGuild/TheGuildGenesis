import { logger } from '../logger.js';

interface RateLimitInfo {
  count: number;
  resetTime: number;
}

export class RateLimiter {
  private limits: Map<string, RateLimitInfo> = new Map();
  private readonly maxRequests: number;
  private readonly windowMs: number;

  constructor(maxRequests: number = 10, windowMs: number = 60000) {
    this.maxRequests = maxRequests;
    this.windowMs = windowMs;
  }

  checkLimit(userId: string): boolean {
    const now = Date.now();
    const userLimit = this.limits.get(userId);

    if (!userLimit || now > userLimit.resetTime) {
      this.limits.set(userId, {
        count: 1,
        resetTime: now + this.windowMs,
      });
      return true;
    }

    if (userLimit.count >= this.maxRequests) {
      logger.debug(`Rate limited user ${userId}`);
      return false;
    }

    userLimit.count++;
    return true;
  }

  cleanup(): void {
    const now = Date.now();
    for (const [userId, limit] of this.limits.entries()) {
      if (now > limit.resetTime) {
        this.limits.delete(userId);
      }
    }
  }
}

export const commandRateLimiter = new RateLimiter(10, 60000);