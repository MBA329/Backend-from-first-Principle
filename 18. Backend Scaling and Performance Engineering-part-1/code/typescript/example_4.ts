import { Pool } from 'pg';

export async function getPostsWithAuthors(db: Pool) {
    const postsRes = await db.query('SELECT * FROM posts');
    const posts = postsRes.rows;

    const authorIDSet = new Set(posts.map(p => p.author_id));
    const authorIDs = Array.from(authorIDSet);
    
    if (authorIDs.length === 0) return [];

    const authorsRes = await db.query('SELECT * FROM authors WHERE id = ANY($1)', [authorIDs]);
    const authors = authorsRes.rows;

    const authorMap = new Map();
    authors.forEach(a => authorMap.set(a.id, a));

    return posts.map(p => ({
        post: p,
        author: authorMap.get(p.author_id)
    }));
}
