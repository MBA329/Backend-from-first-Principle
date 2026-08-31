use actix_web::{web, App, HttpServer, HttpResponse};
use std::time::Duration;
use tokio::signal;
use std::sync::Arc;

// Mock definitions
struct Database;
impl Database {
    async fn close(&self) {}
}

struct JobServer;
impl JobServer {
    async fn shutdown(&self) {}
}

async fn connect_database() -> Database {
    Database
}

async fn start_background_jobs() -> JobServer {
    JobServer
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // --- Startup phase: acquire resources in order ---
    let db = Arc::new(connect_database().await);           // 1. acquire DB (TCP pool)
    let jobs = Arc::new(start_background_jobs().await);     // 2. acquire Redis-backed worker

    let srv = HttpServer::new(|| {
        App::new().route("/", web::get().to(|| async { HttpResponse::Ok().body("OK") }))
    })
    .bind(("127.0.0.1", 8080))?
    // Run the server
    .run();
    
    let server_handle = srv.handle();

    tokio::spawn(async move {
        println!("server started, ready to accept requests");
        if let Err(e) = srv.await {
            eprintln!("listen error: {}", e);
        }
    });

    // Register a handler that waits for SIGINT (Ctrl+C) or SIGTERM (PM2/k8s).
    // We handle BOTH the same way, the intention is identical: shut down.
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Block here until a signal arrives (the "living" phase).
    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    
    println!("signal received, starting graceful shutdown");

    // gracefulShutdown releases resources in REVERSE order of acquisition.
    // Acquired: DB -> jobs -> HTTP server. Released: HTTP -> jobs -> DB.
    
    // 1. CONNECTION DRAINING: stop accepting NEW
    //    connections and waits for in-flight requests to finish
    println!("draining HTTP connections...");
    server_handle.stop(true).await;

    // 2. Stop the background job server (closes Redis connections,
    //    waits for workers to finish current jobs).
    println!("stopping background job server...");
    jobs.shutdown().await;

    // 3. Close the database LAST, finish/commit open transactions,
    //    then close all pooled TCP connections one by one.
    println!("closing database connection...");
    db.close().await;

    println!("server exited properly");
    
    Ok(())
}
