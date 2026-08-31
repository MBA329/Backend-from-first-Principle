use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/health")]
async fn health(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" })),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "reason": "database unreachable"
        }))
    }
}
