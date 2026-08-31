import express, { Request, Response, NextFunction } from 'express';

const app = express();

function securityHeaders(req: Request, res: Response, next: NextFunction) {
    res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');
    res.setHeader('Content-Security-Policy', "default-src 'self'");
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('X-Content-Type-Options', 'nosniff');
    next(); // headers MUST be set before this call
}

app.use(securityHeaders);
// app.listen(8080);
