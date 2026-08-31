use rand::RngCore;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub role: String,
}

pub struct AppState {
    pub rdb: Client,
}

// hash once at signup; store hash, never the password
pub fn hash_password(pw: &str) -> Result<String, bcrypt::BcryptError> {
    hash(pw, DEFAULT_COST)
}

// bcrypt::verify is constant-time internally
pub fn check_password(hash_str: &str, pw: &str) -> bool {
    verify(pw, hash_str).unwrap_or(false)
}

pub fn new_session_id() -> String {
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b); // cryptographically random
    hex::encode(b)
}

use actix_web::{cookie::{Cookie, SameSite}, HttpResponse};
use actix_web::cookie::time::Duration;

pub async fn login(state: actix_web::web::Data<AppState>, u: User) -> Result<HttpResponse, actix_web::Error> {
    let sid = new_session_id();
    let data = serde_json::to_string(&u)?;
    
    let mut con = state.rdb.get_async_connection().await.map_err(actix_web::error::ErrorInternalServerError)?;
    
    // store {sessionID -> userData} with a 15-minute TTL
    let _: () = con.set_ex(format!("sess:{}", sid), data, 15 * 60).await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    
    let cookie = Cookie::build("sid", sid)
        .http_only(true)            // JS cannot read it
        .secure(true)               // HTTPS only
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(Duration::minutes(15))
        .finish();
        
    Ok(HttpResponse::Ok().cookie(cookie).finish())
}
