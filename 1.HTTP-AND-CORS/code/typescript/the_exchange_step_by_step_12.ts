import express, { Request, Response } from 'express';
import path from 'path';

const app = express();

// express.static or res.sendFile handles Range, 206, 416 and If-Range for you,
// driven by the file's modtime and an optional ETag.
app.get('/download', (req: Request, res: Response) => {
    const filePath = path.resolve(__dirname, 'big.zip');
    res.sendFile(filePath);
});
