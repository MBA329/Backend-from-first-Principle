import express, { Request, Response } from 'express';

const app = express();

const db = {
    find: async (id: string) => {
        if (id !== "1") throw new Error("ErrNotFound");
        return { id, visibleTo: (req: Request) => req.header('Authorization') === 'user' };
    }
};

async function getUser(req: Request, res: Response) {
    if (!req.header('Authorization')) {
        res.status(401).send('login required'); // 401: who are you?
        return;
    }
    
    try {
        const user = await db.find(req.params.id);
        if (!user.visibleTo(req)) {
            res.status(403).send('forbidden'); // 403: not allowed
            return;
        }
        res.status(200).json(user); // 200
    } catch (err: any) {
        if (err.message === "ErrNotFound") {
            res.status(404).send('no such user'); // 404
            return;
        }
        res.status(500).send('Server Error');
    }
}
