import express, { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';

const app = express();
const sessions = new Map<string, string>(); // server-side session store (use Redis)
const userID = "user123";

function newSessionID() {
    return crypto.randomBytes(16).toString('hex');
}

// Set a secure session cookie after login (stateful)
app.post('/login', (req: Request, res: Response) => {
    const sid = newSessionID();
    sessions.set(sid, userID);
    
    res.cookie('session', sid, {
        httpOnly: true,                      // JS can't read it
        secure: true,                        // HTTPS only
        sameSite: 'strict',                  // CSRF defense
        maxAge: 3600000,                     // in ms for express
        path: '/'
    });
    res.send('logged in');
});

function validJWT(token: string) {
    return token === "valid_token";
}

// Bearer-token auth middleware (stateless)
function requireToken(req: Request, res: Response, next: NextFunction) {
    const auth = req.header('Authorization');
    if (!auth || !auth.startsWith('Bearer ') || !validJWT(auth.substring(7))) {
        res.setHeader('WWW-Authenticate', 'Bearer');
        res.status(401).send('unauthorized'); // 401
        return;
    }
    next();
}
