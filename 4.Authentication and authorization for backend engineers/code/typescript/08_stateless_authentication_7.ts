import { Request, Response, NextFunction } from 'express';
import jwt from 'jsonwebtoken';

interface User {
    id: string;
    role: string;
}

const secret = 'keep-this-very-secret';

// verify signature + expiry; returns the claims if valid
function verify(tokenStr: string): any {
    try {
        return jwt.verify(tokenStr, secret, { algorithms: ['HS256'] });
    } catch (err) {
        throw new Error('Invalid signature or expired token');
    }
}

export function requireJWT(req: Request, res: Response, next: NextFunction): void {
    const authHeader = req.headers.authorization;
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
        res.status(401).send('authentication failed');
        return;
    }
    
    try {
        const token = authHeader.substring(7);
        const claims = verify(token);
        
        // no store lookup ,  identity comes from the verified token
        const u: User = { id: claims.sub, role: claims.role };
        
        (req as any).user = u;
        next();
    } catch (err) {
        res.status(401).send('authentication failed');
    }
}
