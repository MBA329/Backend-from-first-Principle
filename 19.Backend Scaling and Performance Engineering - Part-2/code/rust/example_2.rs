use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use redis::aio::Connection;
use std::collections::HashMap;

#[get("/health")]
async fn health(db: web::Data<PgPool>, redis_con: web::Data<std::sync::Mutex<Connection>>) -> impl Responder {
    let mut checks = HashMap::new();
    
    if sqlx::query("SELECT 1").execute(db.get_ref()).await.is_ok() {
        checks.insert("database", "ok");
    } else {
        checks.insert("database", "unreachable");
    }
    
    // Simulate redis ping Check
    // if redis::cmd("PING").query_async(&mut *redis_con.lock().unwrap()).await.is_ok() { ... }
    
    if checks.values().any(|&v| v != "ok") {
        HttpResponse::ServiceUnavailable().json(&checks)
    } else {
        HttpResponse::Ok().json(&checks)
    }
}
