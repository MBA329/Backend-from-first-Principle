import express, { Request, Response } from 'express';

const app = express();

// Permanent move that must keep the method/body -> 308
app.all('/oldRoute/:id', (req: Request, res: Response) => {
    res.redirect(308, `/person/${req.params.id}`); // 308
});

function save(req: Request): string {
    return "new_id";
}

// Post/Redirect/Get -> 303 so a browser refresh won't re-POST the form
app.post('/submitForm', (req: Request, res: Response) => {
    const id = save(req);
    res.redirect(303, `/results/${id}`); // 303 (forces GET)
});
