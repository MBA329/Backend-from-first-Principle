import jwt from 'jsonwebtoken';
import { Redis } from 'ioredis';
import crypto from 'crypto';

// Verify with a PUBLIC key, pinning RS256 (blocks alg:none & key confusion)
export function verifyRS256(tokenStr: string, pub: string | Buffer): any {
    try {
        return jwt.verify(tokenStr, pub, { algorithms: ['RS256'] });
    } catch (err) {
        throw new Error('Invalid signature or expired token');
    }
}

function newSessionId(): string {
    return crypto.randomBytes(32).toString('hex');
}

// Rotate a refresh token; detect reuse of an already-spent one.
export async function refresh(rdb: Redis, presented: string): Promise<string> {
    const family = await rdb.get(`rt:${presented}`);
    if (!family) {
        // not a live token ,  was it a previously-spent one? -> theft
        const fam = await rdb.get(`spent:${presented}`);
        if (fam) {
            await rdb.del(`family:${fam}`); // revoke the whole family
            throw new Error("refresh reuse detected");
        }
        throw new Error("invalid refresh token");
    }
    
    await rdb.del(`rt:${presented}`); // consume the live token
    await rdb.set(`spent:${presented}`, family, 'EX', 14 * 24 * 60 * 60); // remember it
    
    const newRefresh = newSessionId();
    await rdb.set(`rt:${newRefresh}`, family, 'EX', 14 * 24 * 60 * 60);
    return newRefresh;
}
