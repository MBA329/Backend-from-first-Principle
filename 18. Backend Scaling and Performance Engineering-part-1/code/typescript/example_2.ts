import express, { Request, Response, NextFunction } from 'express';

export function timingMiddleware(req: Request, res: Response, next: NextFunction) {
    const start = process.hrtime();
    res.on('finish', () => {
        const diff = process.hrtime(start);
        const duration = (diff[0] * 1e9 + diff[1]) / 1e6; // ms
        console.log(`method=${req.method} path=${req.path} status=${res.statusCode} duration=${duration}ms`);
    });
    next();
}
