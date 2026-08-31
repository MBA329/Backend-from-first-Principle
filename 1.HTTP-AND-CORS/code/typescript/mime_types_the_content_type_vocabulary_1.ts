import express, { Request, Response } from 'express';

const app = express();
app.use(express.json());

interface Note {
    id: number;
    title: string;
    done: boolean;
}

app.post('/api/v1/notes', (req: Request, res: Response) => {
    // 1. read request body (express.json() handles this)
    const inNote: Note = req.body;
    
    if (!inNote || typeof inNote !== 'object') {
        res.status(400).send('invalid JSON');
        return;
    }
    
    // 2. headers FIRST
    res.setHeader('Content-Type', 'application/json');
    // 3. status
    res.status(201);
    
    inNote.id = 42;
    // 4. body LAST
    res.json(inNote);
});

app.listen(8080, () => {
    console.log('Server started on port 8080');
});
