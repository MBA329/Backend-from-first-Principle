import express from 'express';
import { Pool } from 'pg';

const app = express();
const db = new Pool();

app.get('/user', async (req, res) => {
    const userID = req.query.id;
    
    try {
        // This does NOT block the main thread. It yields back to the Event Loop.
        const result = await db.query('SELECT name, email FROM users WHERE id = $1', [userID]);
        if (result.rows.length === 0) {
            return res.status(404).send('not found');
        }
        // Resumes here after DB responds
        res.json(result.rows[0]);
    } catch (err) {
        res.status(500).send('error');
    }
});
