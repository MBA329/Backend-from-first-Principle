// Rust, Safe logging practices
use log::error;

// X UNSAFE, never do this
/*
error!(
    "login_failed email={} password={} api_key={}",
    user.email,         // PII leak
    req.password,       // catastrophic
    cfg.openai_key      // secret leak
);
*/

// OK SAFE, IDs and correlation only
/*
error!(
    "login_failed user_id={} correlation_id={} reason={}",
    user.id,
    req.headers().get("X-Request-ID").unwrap_or(""),
    "invalid_credentials" // generic code, not DB message
);
*/
