import { Request, Response } from 'express';

const bookService = { create: (req: any, id: any) => ({}) }; // sketch

export function createBookHandler(req: Request, res: Response) {
    const payload = req.body;

    // Read the trusted user id FROM THE CONTEXT, not from the body.
    // The auth middleware put it there after verifying the token.
    const userId = res.locals.userId;
    const role = res.locals.role;

    if (role !== "admin" && role !== "user") {
        res.status(403).send("forbidden");
        return;
    }

    // Persist with the SERVER-VERIFIED owner id ,  never the client's.
    const book = bookService.create(payload, userId);
    res.status(201).json(book);
}
