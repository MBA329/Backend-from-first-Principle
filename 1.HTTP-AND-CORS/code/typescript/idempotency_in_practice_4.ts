import express, { Request, Response } from 'express';

const app = express();

// key -> string result. Use Redis with a TTL in production.
const seen = new Map<string, string>();

function charge(req: Request): string {
    return "payment_success";
}

app.post('/charge', (req: Request, res: Response) => {
    const key = req.header('Idempotency-Key');
    if (!key) {
        res.status(400).send('missing Idempotency-Key');
        return;
    }
    
    if (seen.has(key)) {                      // replay: return the stored result
        const cached = seen.get(key);
        res.status(200).send(cached);
        return;
    }
    
    const result = charge(req);               // the real, non-idempotent work
    seen.set(key, result);                    // remember it BEFORE responding
    
    res.status(201).send(result);
});
