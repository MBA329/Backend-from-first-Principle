use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Product { id: String }

pub async fn get_product(id: &str, mut con: redis::aio::Connection) -> redis::RedisResult<Product> {
    let cache_key = format!("product:{}", id);

    // 1. Try cache first
    let cached: Option<String> = con.get(&cache_key).await?;
    if let Some(val) = cached {
        let p: Product = serde_json::from_str(&val).unwrap();
        return Ok(p); // Cache HIT
    }

    // 2. Cache miss, query database
    let p = Product { id: id.to_string() }; // Simulated DB query

    // 3. Store in cache with TTL
    let data = serde_json::to_string(&p).unwrap();
    let _: () = con.set_ex(cache_key, data, 600).await?;

    Ok(p)
}
