import { Request, Response } from 'express';

interface CreateBookRequest {
    title: string;
    author: string;
}

export function createBookHandler(req: Request, res: Response) {
    // Step 1 + 2: extract body and deserialize into a native class/dict.
    const payload = req.body as CreateBookRequest;
    if (!payload || !payload.title) {
        // Deserialization failed -> malformed payload.
        res.status(400).json({ error: "invalid request body" });
        return; // terminate the request here; do not proceed.
    }
    // ... validation, transformation, delegation follow ...
}
