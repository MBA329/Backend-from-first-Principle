import { Request, Response, NextFunction } from 'express';
import Redis from 'ioredis';

const MAX_REQUESTS = 50;
const WINDOW_TIME_SECONDS = 60; // 1 minute

export function rateLimitMiddleware(redis: Redis) {
    return async (req: Request, res: Response, next: NextFunction) => {
        // Extract client IP from X-Forwarded-For header
        // (set by reverse proxy like Nginx or Caddy)
        let clientIP = req.headers['x-forwarded-for'] as string;
        if (!clientIP) {
            clientIP = req.socket.remoteAddress || 'unknown';
        }

        // Redis key: per IP, per minute window
        const currentMinute = Math.floor(Date.now() / 1000 / 60);
        const key = `rate_limit:${clientIP}:${currentMinute}`;

        try {
            // INCR is atomic ,  no race condition even with concurrent requests
            const count = await redis.incr(key);

            // Set TTL on first request of this window (key is new)
            if (count === 1) {
                await redis.expire(key, WINDOW_TIME_SECONDS);
            }

            // Check if limit exceeded
            if (count > MAX_REQUESTS) {
                res.setHeader('Retry-After', '60');
                return res.status(429).send("429 Too Many Requests");
            }

            // Proceed to actual handler
            res.setHeader('X-RateLimit-Remaining', (MAX_REQUESTS - count).toString());
            next();
        } catch (err) {
            return res.status(500).send("Internal Server Error");
        }
    };
}
