import express from 'express';
import { Pool } from 'pg';

const app = express();
const db = new Pool();

app.get('/health', async (req, res) => {
    try {
        await db.query('SELECT 1');
        res.status(200).json({ status: 'healthy' });
    } catch (err) {
        res.status(503).json({ status: 'unhealthy', reason: 'database unreachable' });
    }
});
