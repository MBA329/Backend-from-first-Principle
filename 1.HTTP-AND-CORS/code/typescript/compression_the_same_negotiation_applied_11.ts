import express, { Request, Response } from 'express';
import zlib from 'zlib';

const app = express();
const bigPayload = { data: "large amount of data" };

app.get('/data', (req: Request, res: Response) => {
    const body = JSON.stringify(bigPayload);
    
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Vary', 'Accept-Encoding'); // tell caches the body varies by encoding
    
    const acceptEncoding = req.header('Accept-Encoding') || '';
    
    if (acceptEncoding.includes('gzip')) {
        res.setHeader('Content-Encoding', 'gzip');
        zlib.gzip(body, (err, buffer) => {
            if (!err) {
                res.status(200).send(buffer);
            }
        });
        return;
    }
    
    res.status(200).send(body); // uncompressed fallback for clients that can't decode gzip
});
