import { Request, Response } from 'express';

const bookService = { listBooks: async (sort: string) => [] }; // sketch

export async function listBooksHandler(req: Request, res: Response) {
    let sort = req.query.sort as string;

    // VALIDATION: if present, must be an allowed value.
    if (sort && sort !== "name" && sort !== "date") {
        res.status(400).json({ error: "sort must be 'name' or 'date'" });
        return;
    }

    // TRANSFORMATION: optional param -> inject a default.
    if (!sort) {
        sort = "date";
    }

    try {
        const books = await bookService.listBooks(sort); // delegate
        res.status(200).json(books); // array of books
    } catch (e) {
        res.status(500).json({ error: "could not fetch books" });
    }
}
