import { Request, Response, NextFunction } from "express";

// Basic token bucket implementation
class Limiter {
    tokens = 10; // burst
    lastUpdate = Date.now();

    allow(): boolean {
        const now = Date.now();
        const elapsedSecs = (now - this.lastUpdate) / 1000;
        // 5 requests per second regeneration, up to 10 max
        this.tokens = Math.min(10, this.tokens + elapsedSecs * 5);
        this.lastUpdate = now;

        if (this.tokens >= 1) {
            this.tokens -= 1;
            return true;
        }
        return false;
    }
}

const limiters = new Map<string, Limiter>();

// Per-IP limiter: 5 requests per second, burst of 10
function getIPLimiter(ip: string): Limiter {
    let l = limiters.get(ip);
    if (!l) {
        l = new Limiter();
        limiters.set(ip, l);
    }
    return l;
}

export function rateLimitMiddleware(req: Request, res: Response, next: NextFunction) {
    const ip = req.ip || req.socket.remoteAddress || "unknown";
    if (!getIPLimiter(ip).allow()) {
        res.status(429).send("too many requests");
        return;
    }
    next();
}
