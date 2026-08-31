import express, { Request, Response } from 'express';

const app = express();

// Express router binds method + path. Unmatched routes auto-return 404 (or can be configured for 405).
app.get('/notes', listNotes);            // safe, cacheable read
app.get('/notes/:id', getNote);
app.head('/notes/:id', getNote);         // reuse GET; framework drops the body
app.post('/notes', createNote);          // create (server assigns id)
app.put('/notes/:id', putNote);          // full replace (idempotent)
app.patch('/notes/:id', patchNote);      // partial update
app.delete('/notes/:id', deleteNote);    // remove (idempotent)

const db = {
    find: async (id: string) => { 
        if (id !== "1") throw new Error("ErrNotFound");
        return { id, content: "Note 1" }; 
    }
};

async function getNote(req: Request, res: Response) {
    const id = req.params.id;                    // built-in path params
    try {
        const note = await db.find(id);
        res.setHeader('Content-Type', 'application/json');
        res.status(200).json(note);              // 200
    } catch (err: any) {
        if (err.message === "ErrNotFound") {
            res.status(404).send('Not Found');   // 404
            return;
        }
        res.status(500).send('Server Error');
    }
}

// Dummy functions to satisfy the router
function listNotes(req: Request, res: Response) {}
function createNote(req: Request, res: Response) {}
function putNote(req: Request, res: Response) {}
function patchNote(req: Request, res: Response) {}
function deleteNote(req: Request, res: Response) {}
