use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_REQUESTS: i64 = 50;
const WINDOW_TIME_SECONDS: usize = 60;

pub async fn rate_limit_middleware(
    req: ServiceRequest,
    redis_conn: &mut redis::aio::Connection,
) -> Result<ServiceRequest, Error> {
    // Extract client IP from X-Forwarded-For header
    // (set by reverse proxy like Nginx or Caddy)
    let client_ip = if let Some(ip) = req.headers().get("X-Forwarded-For") {
        ip.to_str().unwrap_or("unknown").to_string()
    } else {
        req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_else(|| "unknown".to_string())
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    // Redis key: per IP, per minute window
    let key = format!("rate_limit:{}:{}", client_ip, now / 60);

    // INCR is atomic ,  no race condition even with concurrent requests
    let count: i64 = match redis_conn.incr(&key, 1).await {
        Ok(c) => c,
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("Internal Server Error")),
    };

    // Set TTL on first request of this window (key is new)
    if count == 1 {
        let _: Result<(), _> = redis_conn.expire(&key, WINDOW_TIME_SECONDS).await;
    }

    // Check if limit exceeded
    if count > MAX_REQUESTS {
        return Err(actix_web::error::ErrorTooManyRequests("429 Too Many Requests"));
    }
    
    Ok(req)
}
