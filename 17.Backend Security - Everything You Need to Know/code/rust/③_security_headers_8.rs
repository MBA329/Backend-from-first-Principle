use actix_web::{http::header, App};
use actix_web::middleware::DefaultHeaders;

pub fn apply_security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add((header::X_FRAME_OPTIONS, "DENY"))
        .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        .add((header::REFERRER_POLICY, "strict-origin-when-cross-origin"))
        .add((header::STRICT_TRANSPORT_SECURITY, "max-age=63072000; includeSubDomains; preload"))
        .add((header::CONTENT_SECURITY_POLICY, "default-src 'self'; script-src 'self'; object-src 'none'"))
}

// Router setup logic inside a block
// App::new().wrap(apply_security_headers())
