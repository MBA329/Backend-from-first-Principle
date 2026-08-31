import express, { Request, Response } from 'express';
import crypto from 'crypto';

const app = express();

function loadResource(): Buffer {
    return Buffer.from("hello world");
}

app.get('/resource', (req: Request, res: Response) => {
    const body = loadResource();
    const sum = crypto.createHash('sha256').update(body).digest('hex');
    const etag = `"${sum.substring(0, 16)}"`; // fingerprint of the content
    
    if (req.header('If-None-Match') === etag) { // client already has this exact version
        res.status(304).end();                  // 304, no body, payload saved
        return;
    }
    
    res.setHeader('ETag', etag);
    res.setHeader('Cache-Control', 'max-age=10');
    res.status(200).send(body);                 // 200 + body
});
