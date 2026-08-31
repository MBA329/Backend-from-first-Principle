use actix_web::{web, HttpResponse, Responder};
use actix_web::cookie::{Cookie, SameSite};
use sqlx::PgPool;
use redis::AsyncCommands;
use serde::Deserialize;
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

// Mock
fn verify_argon2(_password: &str, _hash: &str) -> bool { true }

pub async fn login_handler(
    req: web::Json<LoginRequest>,
    db: web::Data<PgPool>,
    redis: web::Data<redis::Client>,
) -> impl Responder {
    // 1. Validate format (first line of defence)
    if !req.email.contains('@') || req.password.len() < 8 {
        return HttpResponse::BadRequest().body("invalid credentials");
    }

    // 2. Parameterised query, no SQL injection possible
    let record = match sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        req.email
    )
    .fetch_optional(db.get_ref())
    .await {
        Ok(Some(r)) => r,
        _ => {
            // 3. Generic error, never reveal whether email exists
            return HttpResponse::Unauthorized().body("invalid email or password");
        }
    };

    // 3. Generic error, never reveal whether email exists
    if !verify_argon2(&req.password, &record.password_hash) {
        return HttpResponse::Unauthorized().body("invalid email or password");
    }

    // 4. Cryptographically secure session ID
    let mut b = vec![0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut b);
    let session_id = URL_SAFE_NO_PAD.encode(&b);

    // 5. Store session in Redis with metadata
    let mut con = match redis.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("server error"),
    };
    let _: () = match con.set_ex(format!("session:{}", session_id), record.id.to_string(), 7 * 24 * 3600).await {
        Ok(_) => (),
        Err(_) => return HttpResponse::InternalServerError().body("server error"),
    };

    // 6. Secure cookie, HttpOnly, Secure, SameSite=Strict
    let cookie = Cookie::build("session_id", session_id)
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(time::Duration::days(7))
        .finish();

    HttpResponse::Ok().cookie(cookie).finish()
}
