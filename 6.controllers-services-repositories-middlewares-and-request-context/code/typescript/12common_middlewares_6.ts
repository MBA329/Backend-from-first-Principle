import { Request, Response, NextFunction } from 'express';

const ALLOWED_ORIGIN = "https://app.example.com";

export function corsMiddleware(req: Request, res: Response, next: NextFunction) {
    const origin = req.headers.origin; // runtime gives us this
    if (origin === ALLOWED_ORIGIN) {
        res.setHeader("Access-Control-Allow-Origin", origin);
    }
    next(); // pass along; browser blocks if header absent
}

export function authMiddleware(req: Request, res: Response, next: NextFunction) {
    const token = req.headers.authorization;
    try {
        const { userId, role } = verifyToken(token); // Sketch
        // SUCCESS: stash identity in the request locals, then continue.
        res.locals.userId = userId;
        res.locals.role = role;
        next();
    } catch (e) {
        res.status(401).send("unauthorized"); // stop
    }
}

function verifyToken(token: any): any { return {}; } // Sketch
