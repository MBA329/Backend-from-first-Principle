import { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';
import { Redis } from 'ioredis';

// Double-submit: compare the CSRF cookie against the header.
export function checkCSRF(req: Request, res: Response, next: NextFunction): void {
    if (req.method === 'GET' || req.method === 'HEAD') {
        return next();
    }
    
    const cookie = req.cookies?.csrf;
    const header = req.headers['x-csrf-token'];
    
    if (!cookie || !header) {
        res.status(403).send('forbidden');
        return;
    }

    const cBuf = Buffer.from(cookie);
    const hBuf = Buffer.from(header as string);

    if (cBuf.length !== hBuf.length || !crypto.timingSafeEqual(cBuf, hBuf)) {
        res.status(403).send('forbidden');
        return;
    }
    
    next();
}

function newSessionId(): string {
    return crypto.randomBytes(32).toString('hex');
}

// Regenerate the session ID right after a successful login.
export async function regenerateSession(rdb: Redis, oldSID: string): Promise<string> {
    const raw = await rdb.get(`sess:${oldSID}`);
    if (raw) {
        await rdb.del(`sess:${oldSID}`); // kill the pre-login ID
    }
    
    const newSID = newSessionId(); // brand-new value
    if (raw) {
        await rdb.set(`sess:${newSID}`, raw, 'EX', 15 * 60);
    }
    return newSID;
}
