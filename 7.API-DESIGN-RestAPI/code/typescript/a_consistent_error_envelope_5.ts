import { Response } from 'express';

export interface FieldError {
    field: string;
    issue: string;
}

export function writeError(res: Response, status: number, code: string, msg: string, details: FieldError[] = []) {
    const body = {
        error: {
            code: code,
            message: msg,
            details: details,
            requestId: res.locals.requestId // assuming requestID is stored in res.locals by middleware
        }
    };
    return res.status(status).json(body); // SAME shape for every error in the whole API
}

// usage example
function exampleUsage(res: Response) {
    writeError(res, 422, "validation_failed", "Some fields are invalid.", [
        { field: "email", issue: "must be a valid email" },
        { field: "age", issue: "must be >= 0" }
    ]);
}
