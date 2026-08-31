import express from 'express';
import { Pool } from 'pg';
import Redis from 'ioredis';

const app = express();
const db = new Pool();
const rdb = new Redis();

app.get('/health', async (req, res) => {
    const checks: Record<string, string> = {};
    
    try {
        await db.query('SELECT 1');
        checks['database'] = 'ok';
    } catch {
        checks['database'] = 'unreachable';
    }

    try {
        await rdb.ping();
        checks['redis'] = 'ok';
    } catch {
        checks['redis'] = 'unreachable';
    }

    if (Object.values(checks).includes('unreachable')) {
        return res.status(503).json(checks);
    }
    res.json(checks);
});
