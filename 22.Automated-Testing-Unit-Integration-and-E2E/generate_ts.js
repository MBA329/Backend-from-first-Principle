const fs = require('fs');
const files = {
  "anatomy_of_a_test_1.ts": `
// In jest or vitest
function applyDiscount(cartTotal: number, percent: number): number {
    return cartTotal - (cartTotal * percent / 100);
}

describe('applyDiscount', () => {
    it('applies the discount correctly', () => {
        // Arrange
        const total = 200;
        const percent = 10;
        // Act
        const result = applyDiscount(total, percent);
        // Assert
        expect(result).toBe(180);
    });
});
`,
  "unit_tests_2.ts": `
export function isStrongPassword(p: string): boolean {
    if (p.length < 8) return false;
    let hasDigit = false;
    let hasUpper = false;
    for (const char of p) {
        if (char >= '0' && char <= '9') hasDigit = true;
        if (char >= 'A' && char <= 'Z') hasUpper = true;
    }
    return hasDigit && hasUpper;
}

describe('isStrongPassword', () => {
    it('rejects short passwords', () => {
        expect(isStrongPassword('short1A')).toBe(false);
    });
    it('accepts valid passwords', () => {
        expect(isStrongPassword('longEnough9')).toBe(true);
    });
});
`,
  "test_doubles_3.ts": `
interface UserRepo {
    findById(id: string): Promise<{email: string, name: string}>;
}
interface Notifier {
    send(to: string, msg: string): Promise<void>;
}

async function greet(repo: UserRepo, notifier: Notifier, id: string) {
    const u = await repo.findById(id);
    await notifier.send(u.email, 'Hi ' + u.name);
}

describe('greet', () => {
    it('sends email', async () => {
        const stubRepo: UserRepo = {
            findById: async () => ({ email: 'a@x.com', name: 'Ada' })
        };
        const spyNotifier = {
            calls: 0,
            lastTo: '',
            send: async (to: string) => {
                spyNotifier.calls++;
                spyNotifier.lastTo = to;
            }
        };

        await greet(stubRepo, spyNotifier, 'u1');

        expect(spyNotifier.calls).toBe(1);
        expect(spyNotifier.lastTo).toBe('a@x.com');
    });
});
`,
  "dependency_injection_for_testability_4.ts": `
interface Order { total: number; }
interface OrderRepo {
    save(o: Order): Promise<void>;
}

class OrderService {
    constructor(private repo: OrderRepo) {}

    async place(o: Order): Promise<void> {
        if (o.total <= 0) throw new Error("invalid total");
        await this.repo.save(o);
    }
}

describe('OrderService', () => {
    it('rejects invalid total', async () => {
        const fakeRepo: OrderRepo = {
            saved: [],
            save: async function(o: Order) { this.saved.push(o); }
        } as any;

        const svc = new OrderService(fakeRepo);
        await expect(svc.place({ total: 0 })).rejects.toThrow("invalid total");
    });
});
`,
  "table_driven_parametrized_tests_5.ts": `
describe('applyDiscount table driven', () => {
    const cases = [
        { name: "ten percent off 200", subtotal: 200, percent: 10, want: 180 },
        { name: "zero discount", subtotal: 100, percent: 0, want: 100 },
        { name: "full discount", subtotal: 100, percent: 100, want: 0 },
        { name: "rounds down", subtotal: 99, percent: 10, want: 89.1 }
    ];

    it.each(cases)('$name', ({ subtotal, percent, want }) => {
        expect(subtotal - (subtotal * percent / 100)).toBe(want);
    });
});
`,
  "testing_with_a_real_database_6.ts": `
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
`,
  "testing_http_handlers_apis_7.ts": `
import request from 'supertest';
import express from 'express';

const app = express();
app.use(express.json());
app.post('/api/v1/users', (req, res) => {
    res.status(201).json({ id: 1 });
});

describe('POST /api/v1/users', () => {
    it('returns 201', async () => {
        const response = await request(app)
            .post('/api/v1/users')
            .send({ email: 'a@x.com', name: 'Ada' })
            .set('Content-Type', 'application/json');

        expect(response.status).toBe(201);
        expect(response.headers['content-type']).toMatch(/application\/json/);
    });
});
`,
  "fixtures_factories_8.ts": `
function newUser(opts: any = {}) {
    return {
        id: "u1",
        email: "default@x.com",
        active: true,
        ...opts
    };
}

describe('newUser', () => {
    it('overrides defaults', () => {
        const u = newUser({ email: 'ada@x.com' });
        expect(u.email).toBe('ada@x.com');
    });
});
`,
  "mocking_time_randomness_external_apis_9.ts": `
import nock from 'nock';
import axios from 'axios';

describe('FetchRate', () => {
    it('fetches rate from external API', async () => {
        nock('https://api.example.com')
            .get('/rates')
            .reply(200, { usd_inr: 83.2 });

        const response = await axios.get('https://api.example.com/rates');
        expect(response.data.usd_inr).toBe(83.2);
    });
});
`,
  "performance_load_testing_10.ts": `
// For performance testing in TS/JS one might use autocannon or k6
import autocannon from 'autocannon';

async function runBenchmark() {
    const result = await autocannon({
        url: 'http://localhost:3000',
        connections: 10,
        pipelining: 1,
        duration: 10
    });
    console.log(result);
}
`
};

for (const [name, content] of Object.entries(files)) {
    fs.writeFileSync('code/typescript/' + name, content.trim() + '\n');
}
