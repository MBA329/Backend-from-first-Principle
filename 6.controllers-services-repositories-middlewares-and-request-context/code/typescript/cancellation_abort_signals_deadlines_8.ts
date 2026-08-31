import { Request, Response, NextFunction } from 'express';
import { v4 as uuidv4 } from 'uuid';

export function requestIdMiddleware(req: Request, res: Response, next: NextFunction) {
    const rid = uuidv4(); // one unique id for this request
    res.locals.requestId = rid;
    console.log(`[${rid}] ${req.method} ${req.path}`);
    
    // Express res object headers
    res.setHeader("X-Request-ID", rid); // echo it back / forward it
    
    next();
}
