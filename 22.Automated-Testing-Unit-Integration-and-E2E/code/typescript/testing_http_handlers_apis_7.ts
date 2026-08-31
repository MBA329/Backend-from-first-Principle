import request from 'supertest';
import express from 'express';

const app = express();
app.use(express.json());
app.post('/api/v1/users', (req, res) => {
    res.status(201).json({ id: 1 });
});

describe('POST /api/v1/users', () => {
    it('returns 201', async () => {
        const response = await request(app)
            .post('/api/v1/users')
            .send({ email: 'a@x.com', name: 'Ada' })
            .set('Content-Type', 'application/json');

        expect(response.status).toBe(201);
        expect(response.headers['content-type']).toMatch(/application/json/);
    });
});
