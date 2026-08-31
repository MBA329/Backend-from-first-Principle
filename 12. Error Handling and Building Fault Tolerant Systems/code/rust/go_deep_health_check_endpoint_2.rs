// Rust, Deep health check endpoint
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use redis::aio::ConnectionManager;
use serde::Serialize;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub checks: HashMap<String, String>,
}

pub async fn health_handler(
    db: web::Data<PgPool>,
    rdb: web::Data<ConnectionManager>
) -> impl Responder {
    let mut checks = HashMap::new();
    let mut overall = "ok".to_string();

    // DB check
    let db_ping = async { sqlx::query("SELECT 1").execute(db.get_ref()).await };
    match timeout(Duration::from_secs(2), db_ping).await {
        Ok(Ok(_)) => {
            checks.insert("database".to_string(), "ok".to_string());
        }
        Ok(Err(e)) => {
            checks.insert("database".to_string(), format!("unhealthy: {}", e));
            overall = "degraded".to_string();
        }
        Err(e) => {
            checks.insert("database".to_string(), format!("unhealthy: {}", e));
            overall = "degraded".to_string();
        }
    }

    // Redis check
    let mut conn = rdb.get_ref().clone();
    match redis::cmd("PING").query_async::<_, String>(&mut conn).await {
        Ok(_) => {
            checks.insert("cache".to_string(), "ok".to_string());
        }
        Err(e) => {
            checks.insert("cache".to_string(), format!("unhealthy: {}", e));
            overall = "degraded".to_string();
        }
    }

    let status = if overall != "ok" {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    } else {
        actix_web::http::StatusCode::OK
    };

    HttpResponse::build(status).json(HealthStatus { status: overall, checks })
}
