import { Request, Response } from 'express';
import bcrypt from 'bcrypt';

interface User {
    id: string;
    hash: string;
}

// dummyHash: a fixed, precomputed bcrypt hash, used to equalize timing.
const dummyHash = '$2a$12$abcdefghijklmnopqrstuv0000000000000000000000000000000000';

function lookupByEmail(email: string): User | null {
    // DB lookup mock
    return null;
}

export async function authenticate(req: Request, res: Response): Promise<void> {
    const { email, pw } = req.body;
    const start = Date.now();
    const floor = 250; // equalize every path to at least 250ms

    try {
        const user = lookupByEmail(email);
        
        if (!user) {
            // still run a dummy hash so step-3 cost is paid anyway
            await bcrypt.compare(pw, dummyHash);
            res.status(401).send('authentication failed');
            return;
        }
        
        // bcrypt.compare is constant-time internally
        const match = await bcrypt.compare(pw, user.hash);
        if (!match) {
            res.status(401).send('authentication failed');
            return;
        }
        
        // success: issue session / JWT ...
        res.send('ok');
    } finally {
        // pad so all outcomes take ~the same time
        const duration = Date.now() - start;
        if (duration < floor) {
            await new Promise(resolve => setTimeout(resolve, floor - duration));
        }
    }
}
