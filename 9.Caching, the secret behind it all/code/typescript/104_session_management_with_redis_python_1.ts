import Redis from 'ioredis';

export interface Product {
    id: string;
    name: string;
    price: number;
}

const redis = new Redis({
    host: 'localhost',
    port: 6379,
});

// GetProduct implements Cache-Aside (Lazy) caching.
// 1. Check Redis first
// 2. On miss, fetch from DB, store in cache, return
export async function getProduct(productId: string): Promise<Product> {
    const cacheKey = `product:${productId}`;

    // Step 1: Try cache first (Cache Hit path)
    const cached = await redis.get(cacheKey);
    if (cached) {
        const product: Product = JSON.parse(cached);
        console.log("[CACHE HIT]", productId);
        return product;
    }

    // Step 2: Cache Miss ,  fetch from database (expensive operation)
    console.log("[CACHE MISS] fetching from DB...", productId);
    const product = await fetchFromDatabase(productId); // simulate DB call

    // Step 3: Store in cache with a 1-hour TTL
    await redis.set(cacheKey, JSON.stringify(product), 'EX', 3600);

    return product;
}

// Write-Through: update DB and cache simultaneously
export async function updateProduct(product: Product): Promise<void> {
    // Step 1: Update in database
    await updateInDatabase(product);

    // Step 2: Write-through ,  update cache immediately
    const cacheKey = `product:${product.id}`;
    await redis.set(cacheKey, JSON.stringify(product), 'EX', 3600);

    console.log("[WRITE-THROUGH] DB + cache updated for", product.id);
}

// Mocks
async function fetchFromDatabase(id: string): Promise<Product> { return { id, name: "Product", price: 10 }; }
async function updateInDatabase(product: Product): Promise<void> {}
