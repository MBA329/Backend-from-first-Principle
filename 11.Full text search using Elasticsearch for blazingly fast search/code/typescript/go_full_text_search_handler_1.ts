// TypeScript, Full text search handler using PostgreSQL
import express, { Request, Response } from 'express';
import { Pool } from 'pg';

interface Product {
    id: number;
    name: string;
    description: string;
    rank: number;
}

export function searchHandler(pool: Pool) {
    return async (req: Request, res: Response): Promise<void> => {
        const query = req.query.q as string;
        if (!query) {
            res.status(400).send("missing query param q");
            return;
        }

        // plainto_tsquery converts plain text safely (no special chars needed)
        // websearch_to_tsquery also supports AND/OR/-term syntax
        const sql = `
            SELECT id, name, description,
                   ts_rank(search_vec, plainto_tsquery('english', $1)) AS rank
            FROM   products
            WHERE  search_vec @@ plainto_tsquery('english', $1)
            ORDER  BY rank DESC
            LIMIT  20
        `;

        try {
            const { rows } = await pool.query<Product>(sql, [query]);
            res.setHeader("Content-Type", "application/json");
            res.send(`found ${rows.length} results\n`);
        } catch (err: any) {
            res.status(500).send(err.message);
        }
    };
}

if (require.main === module) {
    const pool = new Pool({ connectionString: "postgres://..." });
    const app = express();
    app.get("/search", searchHandler(pool));
    app.listen(8080, () => {
        console.log("Server listening on port 8080");
    });
}
