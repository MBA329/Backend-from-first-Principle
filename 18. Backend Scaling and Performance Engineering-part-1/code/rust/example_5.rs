use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let db = PgPoolOptions::new()
        .max_connections(25)
        .min_connections(10) // max idle connections kept ready
        .max_lifetime(Duration::from_secs(5 * 60)) // recycle connections after 5 min
        .idle_timeout(Duration::from_secs(60)) // close idle connections after 1 min
        .connect("postgres://...")
        .await?;
        
    Ok(())
}
