import { Request, Response, NextFunction } from 'express';
import { Redis } from 'ioredis';

interface User {
    id: string;
    role: string;
}

const rdb = new Redis({ host: 'localhost', port: 6379 });

// RequireSession reads the sid cookie and looks it up in Redis.
export function requireSession(req: Request, res: Response, next: NextFunction): void {
    const sid = req.cookies?.sid;
    if (!sid) {
        res.status(401).send('authentication failed');
        return;
    }
    
    rdb.get(`sess:${sid}`).then(raw => {
        if (!raw) { // missing or expired
            res.status(401).send('authentication failed');
            return;
        }
        
        try {
            const u: User = JSON.parse(raw);
            // attach the user to the request context for later handlers
            (req as any).user = u;
            next();
        } catch (err) {
            res.status(401).send('authentication failed');
        }
    }).catch(() => {
        res.status(401).send('authentication failed');
    });
}
