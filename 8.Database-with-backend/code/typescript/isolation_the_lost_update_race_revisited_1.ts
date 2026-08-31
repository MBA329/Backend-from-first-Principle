import { Pool } from 'pg';

export async function transferFunds(pool: Pool, a: string, b: string): Promise<void> {
    const client = await pool.connect();
    
    try {
        await client.query('BEGIN');
        
        // safety net happens in catch block in TS/JS instead of defer
        
        await client.query('UPDATE accounts SET balance = balance - 500 WHERE id = $1', [a]);
        // if this throws -> catch block fires -> A is untouched
        
        await client.query('UPDATE accounts SET balance = balance + 500 WHERE id = $1', [b]);
        // if this throws -> catch block fires -> both reverted
        
        await client.query('COMMIT'); // only here do both writes become permanent
    } catch (err) {
        await client.query('ROLLBACK');
        throw err;
    } finally {
        client.release();
    }
}
