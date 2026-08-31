use serde::Deserialize;
use std::env;
use dotenv::dotenv;
use std::fmt;

// Config holds all runtime application settings.
// The `deserialize` tags enforce rules at startup, this is the
// single most important safeguard for config management.
#[derive(Debug, Deserialize)]
pub struct Config {
    // Application settings
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_env")]
    pub env: String,

    // Database config (sensitive)
    pub db_host: String,
    #[serde(default = "default_db_port")]
    pub db_port: u16,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
    #[serde(default = "default_db_pool_size")]
    pub db_pool_size: u32,

    // External services (sensitive)
    pub stripe_api_key: String,

    // Feature flags (optional, default false)
    #[serde(default)]
    pub new_checkout_enabled: bool,
}

fn default_port() -> u16 { 8080 }
fn default_log_level() -> String { "info".to_string() }
fn default_env() -> String { "development".to_string() }
fn default_db_port() -> u16 { 5432 }
fn default_db_pool_size() -> u32 { 10 }

impl Config {
    // DatabaseURL constructs the connection URL from the parts.
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db_user, self.db_password, self.db_host, self.db_port, self.db_name
        )
    }

    // Load reads env vars, applies defaults, then VALIDATES before returning.
    // Called once at startup, fail loudly here, never silently in production.
    pub fn load() -> Result<Self, envy::Error> {
        // In local dev, load .env into the OS environment.
        // In production this is a no-op (vars already injected by the platform).
        dotenv().ok();

        // THE critical step, validate everything before the app boots
        envy::from_env::<Config>()
    }
}

// Usage in main.rs:
//   fn main() {
//       let cfg = Config::load().expect("config validation failed"); // crash early, crash loud
//   }
