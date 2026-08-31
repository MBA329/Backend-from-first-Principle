use actix_web::{App, HttpServer, HttpResponse};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/", actix_web::web::get().to(|| async { HttpResponse::Ok().finish() }))
    })
    .bind("127.0.0.1:8080")?
    .client_request_timeout(Duration::from_secs(5))   // ReadTimeout
    .client_disconnect_timeout(Duration::from_secs(10)) // WriteTimeout equivalent 
    .keep_alive(Duration::from_secs(60))              // IdleTimeout: how long to hold an idle keep-alive conn
    // Actix handles ReadHeaderTimeout conceptually inside request parsing limits
    .run()
    .await
}
