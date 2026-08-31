use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Serialize)]
struct User { name: String, email: String }

async fn handle_get_user(db: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let user_id = query.get("id").unwrap();

    // This BLOCKS the task, but not the OS thread (yields back to Tokio reactor).
    match sqlx::query_as!(User, "SELECT name, email FROM users WHERE id = $1", user_id.parse::<i64>().unwrap_or(0)).fetch_optional(db.get_ref()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
