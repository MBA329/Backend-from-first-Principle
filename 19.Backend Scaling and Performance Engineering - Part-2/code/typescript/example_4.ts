import express from 'express';
import client from 'prom-client';

const app = express();

const requestDuration = new client.Histogram({
    name: 'http_request_duration_seconds',
    help: 'Duration of HTTP requests',
    labelNames: ['method', 'path', 'status'],
});

// Register it
client.register.registerMetric(requestDuration);

app.get('/metrics', async (req, res) => {
    res.set('Content-Type', client.register.contentType);
    res.send(await client.register.metrics());
});
