use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}

// GetProduct implements Cache-Aside (Lazy) caching.
// 1. Check Redis first
// 2. On miss, fetch from DB, store in cache, return
pub async fn get_product(
    redis_conn: &mut redis::aio::Connection,
    product_id: &str,
) -> Result<Product, Box<dyn std::error::Error>> {
    let cache_key = format!("product:{}", product_id);

    // Step 1: Try cache first (Cache Hit path)
    let cached: Option<String> = redis_conn.get(&cache_key).await?;
    if let Some(cached_data) = cached {
        let product: Product = serde_json::from_str(&cached_data)?;
        println!("[CACHE HIT] {}", product_id);
        return Ok(product);
    }

    // Step 2: Cache Miss ,  fetch from database (expensive operation)
    println!("[CACHE MISS] fetching from DB... {}", product_id);
    let product = fetch_from_database(product_id).await?; // simulate DB call

    // Step 3: Store in cache with a 1-hour TTL
    let data = serde_json::to_string(&product)?;
    let _: () = redis_conn.set_ex(&cache_key, data, 3600).await?;

    Ok(product)
}

// Write-Through: update DB and cache simultaneously
pub async fn update_product(
    redis_conn: &mut redis::aio::Connection,
    product: &Product,
) -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Update in database
    update_in_database(product).await?;

    // Step 2: Write-through ,  update cache immediately
    let cache_key = format!("product:{}", product.id);
    let data = serde_json::to_string(product)?;
    let _: () = redis_conn.set_ex(&cache_key, data, 3600).await?;

    println!("[WRITE-THROUGH] DB + cache updated for {}", product.id);
    Ok(())
}

// Mocks
async fn fetch_from_database(id: &str) -> Result<Product, Box<dyn std::error::Error>> {
    Ok(Product { id: id.to_string(), name: "Product".to_string(), price: 10.0 })
}
async fn update_in_database(_product: &Product) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
