use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::Level;
use std::env;

pub fn setup_logger() {
    let level = if env::var("APP_ENV").unwrap_or_default() == "development" {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let filter = tracing_subscriber::filter::Targets::new()
        .with_default(level);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().json())
        .with(filter)
        .init();
}
