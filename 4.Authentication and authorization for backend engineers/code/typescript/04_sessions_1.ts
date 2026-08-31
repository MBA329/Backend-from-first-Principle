import crypto from 'crypto';
import { Redis } from 'ioredis';
import bcrypt from 'bcrypt';

const rdb = new Redis({ host: 'localhost', port: 6379 });

export interface User {
    id: string;
    role: string;
}

// hash once at signup; store hash, never the password
export async function hashPassword(pw: string): Promise<string> {
    const saltRounds = 10;
    return await bcrypt.hash(pw, saltRounds);
}

// bcrypt.compare is constant-time internally
export async function checkPassword(hash: string, pw: string): Promise<boolean> {
    return await bcrypt.compare(pw, hash);
}

export function newSessionId(): string {
    const b = crypto.randomBytes(32); // cryptographically random
    return b.toString('hex');
}

export async function login(res: any, u: User): Promise<void> {
    const sid = newSessionId();
    const data = JSON.stringify(u);
    
    // store {sessionID -> userData} with a 15-minute TTL
    await rdb.set(`sess:${sid}`, data, 'EX', 15 * 60);
    
    res.cookie('sid', sid, {
        httpOnly: true,            // JS cannot read it
        secure: true,              // HTTPS only
        sameSite: 'lax',
        path: '/',
        expires: new Date(Date.now() + 15 * 60 * 1000)
    });
}
