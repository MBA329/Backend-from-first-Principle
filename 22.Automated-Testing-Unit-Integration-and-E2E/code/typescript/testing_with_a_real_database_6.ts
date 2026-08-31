import { Client } from 'pg';
import { PostgreSqlContainer } from '@testcontainers/postgresql';

describe('UserRepo', () => {
    let container: any;
    let client: Client;

    beforeAll(async () => {
        container = await new PostgreSqlContainer().start();
        client = new Client({ connectionString: container.getConnectionUri() });
        await client.connect();
        await client.query('CREATE TABLE users (id TEXT, email TEXT)');
    });

    afterAll(async () => {
        await client.end();
        await container.stop();
    });

    it('saves and finds user', async () => {
        await client.query('INSERT INTO users (id, email) VALUES ($1, $2)', ['u1', 'a@x.com']);
        const res = await client.query('SELECT * FROM users WHERE id = $1', ['u1']);
        expect(res.rows[0].email).toBe('a@x.com');
    });
});
