import Redis from 'ioredis';

const rdb = new Redis({ host: 'localhost', port: 6379 });

export async function getProduct(id: string) {
    const cacheKey = "product:" + id;

    // 1. Try cache first
    const cached = await rdb.get(cacheKey);
    if (cached) {
        return JSON.parse(cached); // Cache HIT
    }

    // 2. Cache miss, query database
    // const p = await queryProductFromDB(id);
    const p = { id, name: "Sample" };

    // 3. Store in cache with TTL (10 minutes)
    await rdb.set(cacheKey, JSON.stringify(p), 'EX', 600);

    return p;
}
