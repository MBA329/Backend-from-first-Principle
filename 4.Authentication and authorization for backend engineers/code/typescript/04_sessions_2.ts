import crypto from 'crypto';

const pepper = Buffer.from(process.env.PASSWORD_PEPPER || '', 'utf8'); // secret, NOT in DB

// cost parameters ,  tune so one hash ~250ms
const aTime = 3;         // iterations
const aMemory = 64 * 1024; // 64 MB, memory-hard
const aThreads = 4;
const aKeyLen = 32;

function peppered(pw: string): Buffer {
    const m = crypto.createHmac('sha256', pepper);
    m.update(pw);
    return m.digest();
}

// returns an encoded string: salt + params + hash
export function hashArgon(pw: string): Promise<string> {
    return new Promise((resolve, reject) => {
        const salt = crypto.randomBytes(16); // unique per password
        crypto.argon2id({
            password: peppered(pw),
            salt: salt,
            timeCost: aTime,
            memoryCost: aMemory,
            parallelism: aThreads,
            hashLength: aKeyLen
        }, (err, hash) => {
            if (err) reject(err);
            else {
                resolve(`argon2id$${aTime}$${aMemory}$${aThreads}$${salt.toString('base64')}$${hash.toString('base64')}`);
            }
        });
    });
}

// re-derive with the SAME salt+params, then constant-time compare
export function verifyArgon(encoded: string, pw: string): Promise<boolean> {
    return new Promise((resolve, reject) => {
        const parts = encoded.split('$');
        if (parts.length !== 6 || parts[0] !== 'argon2id') {
            return resolve(false);
        }
        const t = parseInt(parts[1], 10);
        const mem = parseInt(parts[2], 10);
        const th = parseInt(parts[3], 10);
        const salt = Buffer.from(parts[4], 'base64');
        const want = Buffer.from(parts[5], 'base64');

        crypto.argon2id({
            password: peppered(pw),
            salt: salt,
            timeCost: t,
            memoryCost: mem,
            parallelism: th,
            hashLength: want.length
        }, (err, got) => {
            if (err) reject(err);
            else resolve(crypto.timingSafeEqual(got, want)); // constant-time compare
        });
    });
}
