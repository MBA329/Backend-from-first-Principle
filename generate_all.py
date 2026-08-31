import os

files_to_write = {
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_1.ts": """
export function percentile(latencies: number[], p: number): number {
    if (latencies.length === 0) return 0;
    const sorted = [...latencies].sort((a, b) => a - b);
    const rank = (p / 100.0) * (sorted.length - 1);
    const idx = Math.ceil(rank);
    return sorted[idx];
}

const samples = [12, 15, 22, 48, 95, 110, 340, 890, 1200, 4800];
console.log("P50: ", percentile(samples, 50));
console.log("P90: ", percentile(samples, 90));
console.log("P99: ", percentile(samples, 99));
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_1.rs": """
pub fn percentile(latencies: &[std::time::Duration], p: f64) -> std::time::Duration {
    if latencies.is_empty() {
        return std::time::Duration::from_millis(0);
    }
    let mut sorted = latencies.to_vec();
    sorted.sort();
    let rank = (p / 100.0) * (sorted.len() - 1) as f64;
    let idx = rank.ceil() as usize;
    sorted[idx]
}

fn main() {
    let samples = vec![
        std::time::Duration::from_millis(12), std::time::Duration::from_millis(15),
        std::time::Duration::from_millis(22), std::time::Duration::from_millis(48),
        std::time::Duration::from_millis(95), std::time::Duration::from_millis(110),
        std::time::Duration::from_millis(340), std::time::Duration::from_millis(890),
        std::time::Duration::from_millis(1200), std::time::Duration::from_millis(4800),
    ];
    println!("P50: {:?}", percentile(&samples, 50.0));
    println!("P90: {:?}", percentile(&samples, 90.0));
    println!("P99: {:?}", percentile(&samples, 99.0));
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_2.ts": """
import express, { Request, Response, NextFunction } from 'express';

export function timingMiddleware(req: Request, res: Response, next: NextFunction) {
    const start = process.hrtime();
    res.on('finish', () => {
        const diff = process.hrtime(start);
        const duration = (diff[0] * 1e9 + diff[1]) / 1e6; // ms
        console.log(`method=${req.method} path=${req.path} status=${res.statusCode} duration=${duration}ms`);
    });
    next();
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_2.rs": """
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use std::{future::{ready, Ready}, time::Instant};
use futures_util::future::LocalBoxFuture;

pub struct TimingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TimingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TimingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future { ready(Ok(TimingMiddlewareService { service })) }
}

pub struct TimingMiddlewareService<S> { service: S }

impl<S, B> Service<ServiceRequest> for TimingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_owned();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            println!("method={} path={} status={} duration={:?}", method, path, res.status(), duration);
            Ok(res)
        })
    }
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_3.ts": """
import express from 'express';
// Node.js doesn't have a direct equivalent to Go's pprof out of the box,
// but we can use the 'clinic' suite or built-in inspector for profiling.
// This is a placeholder showing how one might start an inspector session.
import inspector from 'inspector';

const app = express();

app.listen(6060, () => {
    // Start the inspector to allow profiling connections (equivalent to exposing debug endpoints)
    inspector.open(9229, 'localhost');
    console.log('App running on port 6060. Inspector open on 9229.');
});
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_3.rs": """
// In Rust, we might use pprof crate to expose similar endpoints.
use actix_web::{web, App, HttpServer, HttpResponse};
// use pprof::ProfilerGuard;

async fn profile() -> HttpResponse {
    // Profiling logic would go here, e.g., generating a flamegraph
    HttpResponse::Ok().body("Profiling endpoint")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This exposes a similar debug endpoint
    HttpServer::new(|| {
        App::new()
            .route("/debug/pprof/profile", web::get().to(profile))
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_4.ts": """
import { Pool } from 'pg';

export async function getPostsWithAuthors(db: Pool) {
    const postsRes = await db.query('SELECT * FROM posts');
    const posts = postsRes.rows;

    const authorIDSet = new Set(posts.map(p => p.author_id));
    const authorIDs = Array.from(authorIDSet);
    
    if (authorIDs.length === 0) return [];

    const authorsRes = await db.query('SELECT * FROM authors WHERE id = ANY($1)', [authorIDs]);
    const authors = authorsRes.rows;

    const authorMap = new Map();
    authors.forEach(a => authorMap.set(a.id, a));

    return posts.map(p => ({
        post: p,
        author: authorMap.get(p.author_id)
    }));
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_4.rs": """
use sqlx::{PgPool, FromRow};
use std::collections::{HashSet, HashMap};

#[derive(FromRow, Clone)]
pub struct Post { pub id: i64, pub author_id: i64 }
#[derive(FromRow, Clone)]
pub struct Author { pub id: i64 }
pub struct PostWithAuthor { pub post: Post, pub author: Option<Author> }

pub async fn get_posts_with_authors(db: &PgPool) -> Result<Vec<PostWithAuthor>, sqlx::Error> {
    let posts: Vec<Post> = sqlx::query_as("SELECT * FROM posts").fetch_all(db).await?;
    
    let author_ids: Vec<i64> = posts.iter()
        .map(|p| p.author_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let authors: Vec<Author> = sqlx::query_as("SELECT * FROM authors WHERE id = ANY($1)")
        .bind(&author_ids)
        .fetch_all(db)
        .await?;

    let author_map: HashMap<i64, Author> = authors.into_iter().map(|a| (a.id, a)).collect();

    let result = posts.into_iter().map(|post| {
        let author = author_map.get(&post.author_id).cloned();
        PostWithAuthor { post, author }
    }).collect();

    Ok(result)
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_5.ts": """
import { Pool } from 'pg';

const db = new Pool({
    connectionString: process.env.DATABASE_URL,
    max: 25,               // max open connections
    idleTimeoutMillis: 60000, // close idle connections after 1 min
    // Node pg doesn't have an exact maxLifetime equivalent natively in connection pool,
    // but max and idleTimeoutMillis are the main knobs.
});

db.on('error', (err) => {
    console.error('Unexpected error on idle client', err);
    process.exit(-1);
});
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_5.rs": """
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
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_6.ts": """
import Redis from 'ioredis';

const rdb = new Redis({ host: 'localhost', port: 6379 });

export async function getProduct(id: string) {
    const cacheKey = "product:" + id;

    // 1. Try cache first
    const cached = await rdb.get(cacheKey);
    if (cached) {
        return JSON.parse(cached); // Cache HIT
    }

    // 2. Cache miss, query database
    // const p = await queryProductFromDB(id);
    const p = { id, name: "Sample" };

    // 3. Store in cache with TTL (10 minutes)
    await rdb.set(cacheKey, JSON.stringify(p), 'EX', 600);

    return p;
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_6.rs": """
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct Product { id: String }

pub async fn get_product(id: &str, mut con: redis::aio::Connection) -> redis::RedisResult<Product> {
    let cache_key = format!("product:{}", id);

    // 1. Try cache first
    let cached: Option<String> = con.get(&cache_key).await?;
    if let Some(val) = cached {
        let p: Product = serde_json::from_str(&val).unwrap();
        return Ok(p); // Cache HIT
    }

    // 2. Cache miss, query database
    let p = Product { id: id.to_string() }; // Simulated DB query

    // 3. Store in cache with TTL
    let data = serde_json::to_string(&p).unwrap();
    let _: () = con.set_ex(cache_key, data, 600).await?;

    Ok(p)
}
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/typescript/example_7.ts": """
import express from 'express';
import { Pool } from 'pg';

const app = express();
const db = new Pool();

app.get('/health', async (req, res) => {
    try {
        await db.query('SELECT 1');
        res.status(200).json({ status: 'healthy' });
    } catch (err) {
        res.status(503).json({ status: 'unhealthy', reason: 'database unreachable' });
    }
});
""",
    "18. Backend Scaling and Performance Engineering-part-1/code/rust/example_7.rs": """
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/health")]
async fn health(db: web::Data<PgPool>) -> impl Responder {
    match sqlx::query("SELECT 1").execute(db.get_ref()).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" })),
        Err(_) => HttpResponse::ServiceUnavailable().json(serde_json::json!({
            "status": "unhealthy",
            "reason": "database unreachable"
        }))
    }
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/typescript/example_1.ts": """
import Redis from 'ioredis';

const rdb = new Redis({ host: 'redis', port: 6379 });

export async function storeSession(sessionID: string, userID: string) {
    await rdb.set("session:" + sessionID, userID, 'EX', 24 * 60 * 60);
}

export async function getSession(sessionID: string) {
    return await rdb.get("session:" + sessionID);
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/rust/example_1.rs": """
use redis::AsyncCommands;

pub async fn store_session(mut con: redis::aio::Connection, session_id: &str, user_id: &str) -> redis::RedisResult<()> {
    con.set_ex(format!("session:{}", session_id), user_id, 24 * 3600).await
}

pub async fn get_session(mut con: redis::aio::Connection, session_id: &str) -> redis::RedisResult<String> {
    con.get(format!("session:{}", session_id)).await
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/typescript/example_2.ts": """
import express from 'express';
import { Pool } from 'pg';
import Redis from 'ioredis';

const app = express();
const db = new Pool();
const rdb = new Redis();

app.get('/health', async (req, res) => {
    const checks: Record<string, string> = {};
    
    try {
        await db.query('SELECT 1');
        checks['database'] = 'ok';
    } catch {
        checks['database'] = 'unreachable';
    }

    try {
        await rdb.ping();
        checks['redis'] = 'ok';
    } catch {
        checks['redis'] = 'unreachable';
    }

    if (Object.values(checks).includes('unreachable')) {
        return res.status(503).json(checks);
    }
    res.json(checks);
});
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/rust/example_2.rs": """
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use redis::aio::Connection;
use std::collections::HashMap;

#[get("/health")]
async fn health(db: web::Data<PgPool>, redis_con: web::Data<std::sync::Mutex<Connection>>) -> impl Responder {
    let mut checks = HashMap::new();
    
    if sqlx::query("SELECT 1").execute(db.get_ref()).await.is_ok() {
        checks.insert("database", "ok");
    } else {
        checks.insert("database", "unreachable");
    }
    
    // Simulate redis ping Check
    // if redis::cmd("PING").query_async(&mut *redis_con.lock().unwrap()).await.is_ok() { ... }
    
    if checks.values().any(|&v| v != "ok") {
        HttpResponse::ServiceUnavailable().json(&checks)
    } else {
        HttpResponse::Ok().json(&checks)
    }
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/typescript/example_3.ts": """
import Redis from 'ioredis';

const rdb = new Redis();

interface EmailJob {
    to: string;
    subject: string;
    template: string;
}

export async function enqueueEmail(job: EmailJob) {
    await rdb.lpush("queue:emails", JSON.stringify(job));
}

export async function emailWorker() {
    while (true) {
        const result = await rdb.brpop("queue:emails", 0);
        if (result) {
            const job: EmailJob = JSON.parse(result[1]);
            // sendEmail(job) takes 300ms, but no user is waiting
        }
    }
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/rust/example_3.rs": """
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EmailJob {
    pub to: String,
    pub subject: String,
    pub template: String,
}

pub async fn enqueue_email(mut con: redis::aio::Connection, job: EmailJob) -> redis::RedisResult<()> {
    let data = serde_json::to_string(&job).unwrap();
    con.lpush("queue:emails", data).await
}

pub async fn email_worker(mut con: redis::aio::Connection) {
    loop {
        let result: redis::RedisResult<Vec<String>> = con.brpop("queue:emails", 0.0).await;
        if let Ok(res) = result {
            if res.len() == 2 {
                let job: EmailJob = serde_json::from_str(&res[1]).unwrap();
                // send_email(job) takes 300ms, but no user is waiting
            }
        }
    }
}
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/typescript/example_4.ts": """
import express from 'express';
import client from 'prom-client';

const app = express();

const requestDuration = new client.Histogram({
    name: 'http_request_duration_seconds',
    help: 'Duration of HTTP requests',
    labelNames: ['method', 'path', 'status'],
});

// Register it
client.register.registerMetric(requestDuration);

app.get('/metrics', async (req, res) => {
    res.set('Content-Type', client.register.contentType);
    res.send(await client.register.metrics());
});
""",
    "19.Backend Scaling and Performance Engineering - Part-2/code/rust/example_4.rs": """
use actix_web::{get, HttpResponse, Responder};
use prometheus::{HistogramVec, HistogramOpts, register_histogram_vec};
use lazy_static::lazy_static;

lazy_static! {
    static ref REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        HistogramOpts::new("http_request_duration_seconds", "Duration of HTTP requests"),
        &["method", "path", "status"]
    ).unwrap();
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    HttpResponse::Ok().body(buffer)
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/typescript/a_typical_go_handler_1.ts": """
import net from 'net';

// Node.js is single-threaded, but handles connections asynchronously via the Event Loop
const server = net.createServer((conn) => {
    // For each connection, Node registers events rather than spawning a new thread
    handleConn(conn);
});

function handleConn(conn: net.Socket) {
    conn.on('data', (d) => { /* read */ });
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/rust/a_typical_go_handler_1.rs": """
use tokio::net::TcpListener;

// Tokio provides async runtime similar to Go's goroutines
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        // Create a NEW task (green thread) for each connection
        tokio::spawn(async move {
            handle_conn(socket).await;
        });
    }
}

async fn handle_conn(socket: tokio::net::TcpStream) {
    // Handle connection
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/typescript/a_typical_go_handler_2.ts": """
import express from 'express';
import { Pool } from 'pg';

const app = express();
const db = new Pool();

app.get('/user', async (req, res) => {
    const userID = req.query.id;
    
    try {
        // This does NOT block the main thread. It yields back to the Event Loop.
        const result = await db.query('SELECT name, email FROM users WHERE id = $1', [userID]);
        if (result.rows.length === 0) {
            return res.status(404).send('not found');
        }
        // Resumes here after DB responds
        res.json(result.rows[0]);
    } catch (err) {
        res.status(500).send('error');
    }
});
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/rust/a_typical_go_handler_2.rs": """
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Serialize;

#[derive(Serialize)]
struct User { name: String, email: String }

async fn handle_get_user(db: web::Data<PgPool>, query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let user_id = query.get("id").unwrap();

    // This BLOCKS the task, but not the OS thread (yields back to Tokio reactor).
    match sqlx::query_as!(User, "SELECT name, email FROM users WHERE id = $1", user_id.parse::<i64>().unwrap_or(0)).fetch_optional(db.get_ref()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("not found"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/typescript/python_asynciolock_for_async_code_3.ts": """
// In Node.js, JS execution is single-threaded, so we don't need Mutexes 
// for synchronous operations on shared memory.
let counter = 0;

async function increment() {
    counter++; // No race condition because JS is single-threaded
}

async function main() {
    const promises = [];
    for (let i = 0; i < 1000; i++) {
        promises.push(increment());
    }
    await Promise.all(promises);
    console.log(counter); // Always 1000
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/rust/python_asynciolock_for_async_code_3.rs": """
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..1000 {
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap(); // Acquire
            *num += 1;
            // Release on drop
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap()); // Always 1000
}
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/typescript/python_asynciolock_for_async_code_4.ts": """
// In JS, actor-like message passing isn't necessary for simple counters,
// but we can simulate a channel with an async queue or stream.
// Here we just use an async generator or a simple Event Emitter.

import { EventEmitter } from 'events';

const ch = new EventEmitter();
let counter = 0;

ch.on('delta', (d) => {
    counter += d;
});

for (let i = 0; i < 1000; i++) {
    ch.emit('delta', 1);
}

// Since emit is synchronous, it's already done
console.log(counter);
""",
    "20.Concurrency & Parallelism - IO Bound vs CPU Bound/code/rust/python_asynciolock_for_async_code_4.rs": """
use std::sync::mpsc;
use std::thread;

fn counter_service(rx: mpsc::Receiver<i32>, done: mpsc::Sender<i32>) {
    let mut counter = 0;
    for delta in rx {
        counter += delta; // Only this thread modifies counter
    }
    done.send(counter).unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();

    thread::spawn(move || {
        counter_service(rx, done_tx);
    });

    for _ in 0..1000 {
        tx.send(1).unwrap();
    }
    drop(tx); // Close the channel

    let result = done_rx.recv().unwrap();
    println!("{}", result); // 1000, no race condition, no mutex needed
}
"""
}

base_dir = "/Users/HP/Documents/code/backend /Backend-from-first-Principle/"
for rel_path, content in files_to_write.items():
    full_path = os.path.join(base_dir, rel_path)
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    with open(full_path, "w") as f:
        f.write(content.strip() + "\n")
print("Files generated successfully.")
