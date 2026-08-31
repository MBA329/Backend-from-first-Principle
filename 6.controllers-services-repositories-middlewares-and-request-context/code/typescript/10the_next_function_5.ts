import { Request, Response, NextFunction } from 'express';

// An Express middleware receives the request, response, and `next`
// function that invokes the rest of the chain.
export function loggingMiddleware(req: Request, res: Response, next: NextFunction) {
    console.log(`${req.method} ${req.path}`); // do work

    // EARLY EXIT example (short-circuit, never calls next):
    if (req.headers["x-blocked"] === "yes") {
        res.status(403).send("forbidden");
        return; // stops here
    }

    next(); // === next(): continue the chain ===
}
