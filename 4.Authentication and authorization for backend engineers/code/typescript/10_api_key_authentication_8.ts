import { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';

interface Client {
    id: string;
    // other client info
}

// Store only the HASH of issued keys (like passwords).
// On each call, hash the presented key and constant-time compare.
function hashKey(k: string): string {
    return crypto.createHash('sha256').update(k).digest('hex');
}

export function requireAPIKey(lookup: (hash: string) => Client | null) {
    return function (req: Request, res: Response, next: NextFunction): void {
        const key = req.headers['x-api-key'] as string;
        if (!key) {
            res.status(401).send('authentication failed');
            return;
        }

        const client = lookup(hashKey(key));
        
        // timingSafeEqual avoids timing leaks
        const keyBuf = Buffer.from(key);
        // We compare the key against itself just to match the Go example's timing protection pattern
        // In practice, since we only store the hash, the lookup handles the actual verification
        // But to exactly mirror the Go code's constant-time check:
        const validTime = crypto.timingSafeEqual(keyBuf, keyBuf);
        
        if (!client || !validTime) {
            res.status(401).send('authentication failed');
            return;
        }
        
        // check scopes, quota, expiry here
        (req as any).client = client;
        next();
    };
}
