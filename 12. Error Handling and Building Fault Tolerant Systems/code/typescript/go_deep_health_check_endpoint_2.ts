// TypeScript, Deep health check endpoint
import express, { Request, Response } from 'express';
import { Pool } from 'pg';
import { Redis } from 'ioredis';

interface HealthStatus {
    status: string;
    checks: Record<string, string>;
}

export function healthHandler(db: Pool, rdb: Redis) {
    return async (req: Request, res: Response) => {
        const checks: Record<string, string> = {};
        let overall = "ok";

        // DB check (with basic timeout simulation via Promise.race)
        try {
            const dbCheck = db.query('SELECT 1');
            const timeout = new Promise((_, reject) => setTimeout(() => reject(new Error("Timeout")), 2000));
            await Promise.race([dbCheck, timeout]);
            checks["database"] = "ok";
        } catch (err: any) {
            checks["database"] = "unhealthy: " + err.message;
            overall = "degraded";
        }

        // Redis check
        try {
            await rdb.ping();
            checks["cache"] = "ok";
        } catch (err: any) {
            checks["cache"] = "unhealthy: " + err.message;
            overall = "degraded";
        }

        const status = overall !== "ok" ? 503 : 200;

        res.setHeader("Content-Type", "application/json");
        res.status(status).json({ status: overall, checks });
    };
}
