use actix_web::cookie::{Cookie, SameSite};
use time::Duration;

pub fn set_session_cookie(session_id: &str) -> Cookie<'static> {
    Cookie::build("session_id", session_id.to_string())
        .http_only(true)           // JS cannot access
        .secure(true)              // HTTPS only
        .same_site(SameSite::Strict) // no cross-site
        .max_age(Duration::days(7)) // 7 days
        .path("/")
        .finish()
}
