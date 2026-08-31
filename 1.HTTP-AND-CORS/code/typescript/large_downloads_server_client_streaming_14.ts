import express, { Request, Response } from 'express';
import multer from 'multer';

const app = express();

// 1. Receive a multipart upload
// up to 32 MB in memory, overflow spills to disk (handled by multer limits/disk storage)
const upload = multer({ dest: 'uploads/', limits: { fileSize: 32 * 1024 * 1024 } });

app.post('/upload', upload.single('file'), (req: Request, res: Response) => {
    const file = req.file;
    if (!file) {
        res.status(400).send('no file');
        return;
    }
    res.send(`received ${file.originalname}`);
});

// 2. Stream a response in chunks (Server-Sent Events)
app.get('/stream', (req: Request, res: Response) => {
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Connection', 'keep-alive');
    
    let i = 0;
    const interval = setInterval(() => {
        res.write(`data: chunk ${i}\n\n`); // write() pushes each chunk to the client immediately
        i++;
        if (i >= 5) {
            clearInterval(interval);
            res.end();
        }
    }, 1000);
});
