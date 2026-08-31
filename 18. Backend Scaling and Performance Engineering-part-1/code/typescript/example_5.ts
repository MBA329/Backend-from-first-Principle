import { Pool } from 'pg';

const db = new Pool({
    connectionString: process.env.DATABASE_URL,
    max: 25,               // max open connections
    idleTimeoutMillis: 60000, // close idle connections after 1 min
    // Node pg doesn't have an exact maxLifetime equivalent natively in connection pool,
    // but max and idleTimeoutMillis are the main knobs.
});

db.on('error', (err) => {
    console.error('Unexpected error on idle client', err);
    process.exit(-1);
});
