// Rust, Config validation at boot
use std::env;

pub struct Config {
    pub database_url: String,
    pub openai_key: String,
    pub jwt_secret: String,
    pub resend_api_key: String,
}

// must_load panics if any required variable is missing.
// Call this once in main() before starting the server.
pub fn must_load() -> Config {
    let required = [
        "DATABASE_URL",
        "OPENAI_API_KEY",
        "JWT_SECRET",
        "RESEND_API_KEY",
    ];

    let missing: Vec<&str> = required.into_iter()
        .filter(|&key| env::var(key).is_err())
        .collect();

    if !missing.is_empty() {
        // Crash immediately, loud and clear
        panic!("[FATAL] missing required env vars: {}", missing.join(", "));
    }

    Config {
        database_url: env::var("DATABASE_URL").unwrap(),
        openai_key: env::var("OPENAI_API_KEY").unwrap(),
        jwt_secret: env::var("JWT_SECRET").unwrap(),
        resend_api_key: env::var("RESEND_API_KEY").unwrap(),
    }
}

// main.rs
fn main() {
    let _cfg = must_load();  // panics here if config invalid
    // let server = new_server(cfg);
    // server.listen();
}
