import express, { Request, Response } from 'express';
import crypto from 'crypto';

const app = express();

function newETag() {
    return crypto.randomBytes(8).toString('hex');
}

const db = {
    get: (id: string) => ({ id, etag: "current_etag", apply: (body: any) => {} }),
    save: (doc: any) => {}
};

app.put('/doc/:id', (req: Request, res: Response) => {
    const doc = db.get(req.params.id);
    const want = req.header('If-Match');          // the ETag the client last saw
    
    if (!want) {
        res.status(400).send('If-Match required');
        return;
    }
    
    if (want !== doc.etag) {                      // someone changed it first -> conflict
        res.status(412).send('version conflict'); // 412
        return;
    }
    
    doc.apply(req.body);
    doc.etag = newETag();                         // bump the version
    db.save(doc);
    
    res.setHeader('ETag', doc.etag);
    res.status(200).end();
});
