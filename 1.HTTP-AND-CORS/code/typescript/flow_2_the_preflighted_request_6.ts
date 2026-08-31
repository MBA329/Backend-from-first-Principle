import express, { Request, Response, NextFunction } from 'express';

const app = express();

function cors(req: Request, res: Response, next: NextFunction) {
    res.setHeader('Access-Control-Allow-Origin', 'https://example.com'); // exact, not * (credentials)
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, PATCH, DELETE');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    res.setHeader('Access-Control-Allow-Credentials', 'true');
    res.setHeader('Access-Control-Max-Age', '86400'); // cache the approval for 24h
    
    if (req.method === 'OPTIONS') {
        res.status(204).end(); // 204: answer the preflight and stop
        return;
    }
    next();
}

app.use(cors);
