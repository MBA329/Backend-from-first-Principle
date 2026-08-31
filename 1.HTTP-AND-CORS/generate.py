import os

base_dir = "/Users/HP/Documents/code/backend /Backend-from-first-Principle/1.HTTP-AND-CORS"
ts_dir = os.path.join(base_dir, "code/typescript")
rs_dir = os.path.join(base_dir, "code/rust")

os.makedirs(ts_dir, exist_ok=True)
os.makedirs(rs_dir, exist_ok=True)

# Define content of each file.
files = {
    "mime_types_the_content_type_vocabulary_1": {
        "ts": """import express, { Request, Response } from 'express';

const app = express();
app.use(express.json());

interface Note {
    id: number;
    title: string;
    done: boolean;
}

app.post('/api/v1/notes', (req: Request, res: Response) => {
    // 1. read request body (express.json() handles this)
    const inNote: Note = req.body;
    
    if (!inNote || typeof inNote !== 'object') {
        res.status(400).send('invalid JSON');
        return;
    }
    
    // 2. headers FIRST
    res.setHeader('Content-Type', 'application/json');
    // 3. status
    res.status(201);
    
    inNote.id = 42;
    // 4. body LAST
    res.json(inNote);
});

app.listen(8080, () => {
    console.log('Server started on port 8080');
});
""",
        "rs": """use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Note {
    id: Option<i32>,
    title: String,
    done: bool,
}

async fn create_note(mut in_note: web::Json<Note>) -> impl Responder {
    // 1. read request body (web::Json handles deserialization)
    
    // 2. headers FIRST (handled by HttpResponse builder)
    // 3. status
    // 4. body LAST
    in_note.id = Some(42);
    
    HttpResponse::Created()
        .content_type("application/json")
        .json(in_note.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/api/v1/notes", web::post().to(create_note))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
"""
    },
    "security_headers_the_attack_each_one_sto_2": {
        "ts": """import express, { Request, Response, NextFunction } from 'express';

const app = express();

function securityHeaders(req: Request, res: Response, next: NextFunction) {
    res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');
    res.setHeader('Content-Security-Policy', "default-src 'self'");
    res.setHeader('X-Frame-Options', 'DENY');
    res.setHeader('X-Content-Type-Options', 'nosniff');
    next(); // headers MUST be set before this call
}

app.use(securityHeaders);
// app.listen(8080);
""",
        "rs": """use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, App, Error, HttpResponse, HttpServer};
use actix_web::http::header;
use futures_util::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::task::{Context, Poll};

// Actix-web provides a built-in middleware for security headers via DefaultHeaders, 
// but here is a custom middleware to match the concept:

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service: Rc::new(service) })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let headers = res.headers_mut();
            headers.insert(header::STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains".parse().unwrap());
            headers.insert(header::CONTENT_SECURITY_POLICY, "default-src 'self'".parse().unwrap());
            headers.insert(header::HeaderName::from_static("x-frame-options"), "DENY".parse().unwrap());
            headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
            Ok(res)
        })
    }
}
// App::new().wrap(SecurityHeaders)
"""
    },
    "wiring_methods_to_handlers_3": {
        "ts": """import express, { Request, Response } from 'express';

const app = express();

// Express router binds method + path. Unmatched routes auto-return 404 (or can be configured for 405).
app.get('/notes', listNotes);            // safe, cacheable read
app.get('/notes/:id', getNote);
app.head('/notes/:id', getNote);         // reuse GET; framework drops the body
app.post('/notes', createNote);          // create (server assigns id)
app.put('/notes/:id', putNote);          // full replace (idempotent)
app.patch('/notes/:id', patchNote);      // partial update
app.delete('/notes/:id', deleteNote);    // remove (idempotent)

const db = {
    find: async (id: string) => { 
        if (id !== "1") throw new Error("ErrNotFound");
        return { id, content: "Note 1" }; 
    }
};

async function getNote(req: Request, res: Response) {
    const id = req.params.id;                    // built-in path params
    try {
        const note = await db.find(id);
        res.setHeader('Content-Type', 'application/json');
        res.status(200).json(note);              // 200
    } catch (err: any) {
        if (err.message === "ErrNotFound") {
            res.status(404).send('Not Found');   // 404
            return;
        }
        res.status(500).send('Server Error');
    }
}

// Dummy functions to satisfy the router
function listNotes(req: Request, res: Response) {}
function createNote(req: Request, res: Response) {}
function putNote(req: Request, res: Response) {}
function patchNote(req: Request, res: Response) {}
function deleteNote(req: Request, res: Response) {}
""",
        "rs": """use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Note {
    id: String,
    content: String,
}

// Actix-web binds method + path. An unmatched method auto-returns 405 Method Not Allowed.
/*
App::new()
    .route("/notes", web::get().to(list_notes))      // safe, cacheable read
    .route("/notes/{id}", web::get().to(get_note))
    .route("/notes/{id}", web::head().to(get_note))  // reuse GET; framework drops the body
    .route("/notes", web::post().to(create_note))    // create (server assigns id)
    .route("/notes/{id}", web::put().to(put_note))   // full replace (idempotent)
    .route("/notes/{id}", web::patch().to(patch_note)) // partial update
    .route("/notes/{id}", web::delete().to(delete_note)) // remove (idempotent)
*/

async fn get_note(id: web::Path<String>) -> impl Responder {
    let id_val = id.into_inner();                // built-in path params
    
    // simulated db.Find
    if id_val != "1" {
        return HttpResponse::NotFound().finish(); // 404
    }
    
    let note = Note {
        id: id_val,
        content: "Note 1".to_string(),
    };
    
    HttpResponse::Ok()                           // 200
        .content_type("application/json")
        .json(note)
}
"""
    },
    "idempotency_in_practice_4": {
        "ts": """import express, { Request, Response } from 'express';

const app = express();

// key -> string result. Use Redis with a TTL in production.
const seen = new Map<string, string>();

function charge(req: Request): string {
    return "payment_success";
}

app.post('/charge', (req: Request, res: Response) => {
    const key = req.header('Idempotency-Key');
    if (!key) {
        res.status(400).send('missing Idempotency-Key');
        return;
    }
    
    if (seen.has(key)) {                      // replay: return the stored result
        const cached = seen.get(key);
        res.status(200).send(cached);
        return;
    }
    
    const result = charge(req);               // the real, non-idempotent work
    seen.set(key, result);                    // remember it BEFORE responding
    
    res.status(201).send(result);
});
""",
        "rs": """use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

// key -> String result. Use Redis with a TTL in production.
struct AppState {
    seen: Mutex<HashMap<String, String>>,
}

fn charge(_req: &HttpRequest) -> String {
    "payment_success".to_string()
}

async fn create_payment(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let key_header = req.headers().get("Idempotency-Key");
    
    let key = match key_header {
        Some(k) => match k.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return HttpResponse::BadRequest().body("invalid Idempotency-Key"),
        },
        None => return HttpResponse::BadRequest().body("missing Idempotency-Key"),
    };
    
    let mut seen = data.seen.lock().unwrap();
    if let Some(cached) = seen.get(&key) {    // replay: return the stored result
        return HttpResponse::Ok().body(cached.clone());
    }
    
    let result = charge(&req);                // the real, non-idempotent work
    seen.insert(key, result.clone());         // remember it BEFORE responding
    
    HttpResponse::Created().body(result)
}
"""
    },
    "two_authentication_architectures_5": {
        "ts": """import express, { Request, Response, NextFunction } from 'express';
import crypto from 'crypto';

const app = express();
const sessions = new Map<string, string>(); // server-side session store (use Redis)
const userID = "user123";

function newSessionID() {
    return crypto.randomBytes(16).toString('hex');
}

// Set a secure session cookie after login (stateful)
app.post('/login', (req: Request, res: Response) => {
    const sid = newSessionID();
    sessions.set(sid, userID);
    
    res.cookie('session', sid, {
        httpOnly: true,                      // JS can't read it
        secure: true,                        // HTTPS only
        sameSite: 'strict',                  // CSRF defense
        maxAge: 3600000,                     // in ms for express
        path: '/'
    });
    res.send('logged in');
});

function validJWT(token: string) {
    return token === "valid_token";
}

// Bearer-token auth middleware (stateless)
function requireToken(req: Request, res: Response, next: NextFunction) {
    const auth = req.header('Authorization');
    if (!auth || !auth.startsWith('Bearer ') || !validJWT(auth.substring(7))) {
        res.setHeader('WWW-Authenticate', 'Bearer');
        res.status(401).send('unauthorized'); // 401
        return;
    }
    next();
}
""",
        "rs": """use actix_web::{web, cookie::{Cookie, SameSite}, dev::{Service, ServiceRequest, ServiceResponse, Transform}, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};
use std::collections::HashMap;
use std::sync::Mutex;

struct AppState {
    sessions: Mutex<HashMap<String, String>>, // server-side session store (use Redis)
}

fn new_session_id() -> String {
    "random_uuid_here".to_string() // mock
}

// Set a secure session cookie after login (stateful)
async fn login(data: web::Data<AppState>) -> impl Responder {
    let sid = new_session_id();
    let user_id = "user123".to_string();
    
    let mut sessions = data.sessions.lock().unwrap();
    sessions.insert(sid.clone(), user_id);
    
    let cookie = Cookie::build("session", sid)
        .http_only(true)                    // JS can't read it
        .secure(true)                       // HTTPS only
        .same_site(SameSite::Strict)        // CSRF defense
        .max_age(actix_web::cookie::time::Duration::seconds(3600))
        .path("/")
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .body("logged in")
}

fn valid_jwt(token: &str) -> bool {
    token == "valid_token"
}

// Bearer-token auth middleware (stateless) - Using Actix Web Middleware Pattern
pub struct RequireToken;

impl<S, B> Transform<S, ServiceRequest> for RequireToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireTokenMiddleware { service: Rc::new(service) })
    }
}

pub struct RequireTokenMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
        
        let valid = match auth {
            Some(auth_str) if auth_str.starts_with("Bearer ") => {
                valid_jwt(&auth_str[7..])
            }
            _ => false,
        };

        if !valid {
            let (req, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .append_header(("WWW-Authenticate", "Bearer"))
                .body("unauthorized")
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let svc = self.service.clone();
        Box::pin(async move {
            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
"""
    },
    "flow_2_the_preflighted_request_6": {
        "ts": """import express, { Request, Response, NextFunction } from 'express';

const app = express();

function cors(req: Request, res: Response, next: NextFunction) {
    res.setHeader('Access-Control-Allow-Origin', 'https://example.com'); // exact, not * (credentials)
    res.setHeader('Access-Control-Allow-Methods', 'GET, POST, PUT, PATCH, DELETE');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    res.setHeader('Access-Control-Allow-Credentials', 'true');
    res.setHeader('Access-Control-Max-Age', '86400'); // cache the approval for 24h
    
    if (req.method === 'OPTIONS') {
        res.status(204).end(); // 204: answer the preflight and stop
        return;
    }
    next();
}

app.use(cors);
""",
        "rs": """use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use actix_web::http::Method;
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};

// Actix-web provides actix-cors, but here is a custom middleware to match the concept:
pub struct Cors;

impl<S, B> Transform<S, ServiceRequest> for Cors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorsMiddleware { service: Rc::new(service) })
    }
}

pub struct CorsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CorsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let is_options = req.method() == Method::OPTIONS;

        if is_options {
            let (req, _pl) = req.into_parts();
            let mut res = HttpResponse::NoContent().finish(); // 204: answer the preflight and stop
            let headers = res.headers_mut();
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://example.com".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());
            
            return Box::pin(async { Ok(ServiceResponse::new(req, res.map_into_right_body())) });
        }

        let svc = self.service.clone();
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let headers = res.headers_mut();
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://example.com".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());
            Ok(res.map_into_left_body())
        })
    }
}
"""
    },
    "status_codes_7": {
        "ts": """import express, { Request, Response } from 'express';

const app = express();

const db = {
    find: async (id: string) => {
        if (id !== "1") throw new Error("ErrNotFound");
        return { id, visibleTo: (req: Request) => req.header('Authorization') === 'user' };
    }
};

async function getUser(req: Request, res: Response) {
    if (!req.header('Authorization')) {
        res.status(401).send('login required'); // 401: who are you?
        return;
    }
    
    try {
        const user = await db.find(req.params.id);
        if (!user.visibleTo(req)) {
            res.status(403).send('forbidden'); // 403: not allowed
            return;
        }
        res.status(200).json(user); // 200
    } catch (err: any) {
        if (err.message === "ErrNotFound") {
            res.status(404).send('no such user'); // 404
            return;
        }
        res.status(500).send('Server Error');
    }
}
""",
        "rs": """use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: String,
}

impl User {
    fn visible_to(&self, req: &HttpRequest) -> bool {
        if let Some(auth) = req.headers().get("Authorization") {
            return auth.to_str().unwrap_or("") == "user";
        }
        false
    }
}

async fn get_user(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    if req.headers().get("Authorization").is_none() {
        return HttpResponse::Unauthorized().body("login required"); // 401: who are you?
    }
    
    let id_val = id.into_inner();
    
    // simulated db.Find
    if id_val != "1" {
        return HttpResponse::NotFound().body("no such user");       // 404
    }
    
    let user = User { id: id_val };
    
    if !user.visible_to(&req) {
        return HttpResponse::Forbidden().body("forbidden");         // 403: not allowed
    }
    
    HttpResponse::Ok().json(user)                                   // 200
}
"""
    },
    "redirections_8": {
        "ts": """import express, { Request, Response } from 'express';

const app = express();

// Permanent move that must keep the method/body -> 308
app.all('/oldRoute/:id', (req: Request, res: Response) => {
    res.redirect(308, `/person/${req.params.id}`); // 308
});

function save(req: Request): string {
    return "new_id";
}

// Post/Redirect/Get -> 303 so a browser refresh won't re-POST the form
app.post('/submitForm', (req: Request, res: Response) => {
    const id = save(req);
    res.redirect(303, `/results/${id}`); // 303 (forces GET)
});
""",
        "rs": """use actix_web::{web, HttpRequest, HttpResponse, Responder};

// Permanent move that must keep the method/body -> 308
async fn old_route(id: web::Path<String>) -> impl Responder {
    let target = format!("/person/{}", id.into_inner());
    HttpResponse::PermanentRedirect() // 308
        .insert_header(("Location", target))
        .finish()
}

fn save(_req: &HttpRequest) -> String {
    "new_id".to_string()
}

// Post/Redirect/Get -> 303 so a browser refresh won't re-POST the form
async fn submit_form(req: HttpRequest) -> impl Responder {
    let id = save(&req);
    let target = format!("/results/{}", id);
    HttpResponse::SeeOther() // 303 (forces GET)
        .insert_header(("Location", target))
        .finish()
}
"""
    },
    "layer_2_validation_via_etag_last_modifie_9": {
        "ts": """import express, { Request, Response } from 'express';
import crypto from 'crypto';

const app = express();

function loadResource(): Buffer {
    return Buffer.from("hello world");
}

app.get('/resource', (req: Request, res: Response) => {
    const body = loadResource();
    const sum = crypto.createHash('sha256').update(body).digest('hex');
    const etag = `"${sum.substring(0, 16)}"`; // fingerprint of the content
    
    if (req.header('If-None-Match') === etag) { // client already has this exact version
        res.status(304).end();                  // 304, no body, payload saved
        return;
    }
    
    res.setHeader('ETag', etag);
    res.setHeader('Cache-Control', 'max-age=10');
    res.status(200).send(body);                 // 200 + body
});
""",
        "rs": """use actix_web::{HttpRequest, HttpResponse, Responder};
use sha2::{Sha256, Digest};
use hex;

fn load_resource() -> Vec<u8> {
    b"hello world".to_vec()
}

async fn get_resource(req: HttpRequest) -> impl Responder {
    let body = load_resource();
    
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let result = hasher.finalize();
    let etag = format!("\"{}\"", hex::encode(&result[..8])); // fingerprint of the content
    
    if let Some(if_none_match) = req.headers().get("If-None-Match") {
        if if_none_match.to_str().unwrap_or("") == etag {   // client already has this exact version
            return HttpResponse::NotModified().finish();    // 304, no body, payload saved
        }
    }
    
    HttpResponse::Ok()
        .insert_header(("ETag", etag))
        .insert_header(("Cache-Control", "max-age=10"))
        .body(body)                                         // 200 + body
}
"""
    },
    "the_optimistic_locking_flow_10": {
        "ts": """import express, { Request, Response } from 'express';
import crypto from 'crypto';

const app = express();

function newETag() {
    return crypto.randomBytes(8).toString('hex');
}

const db = {
    get: (id: string) => ({ id, etag: "current_etag", apply: (body: any) => {} }),
    save: (doc: any) => {}
};

app.put('/doc/:id', (req: Request, res: Response) => {
    const doc = db.get(req.params.id);
    const want = req.header('If-Match');          // the ETag the client last saw
    
    if (!want) {
        res.status(400).send('If-Match required');
        return;
    }
    
    if (want !== doc.etag) {                      // someone changed it first -> conflict
        res.status(412).send('version conflict'); // 412
        return;
    }
    
    doc.apply(req.body);
    doc.etag = newETag();                         // bump the version
    db.save(doc);
    
    res.setHeader('ETag', doc.etag);
    res.status(200).end();
});
""",
        "rs": """use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn new_etag() -> String {
    "new_random_etag".to_string()
}

struct Doc {
    id: String,
    etag: String,
}

impl Doc {
    fn apply(&mut self, _body: &web::Bytes) {}
}

struct Db;
impl Db {
    fn get(&self, id: String) -> Doc {
        Doc { id, etag: "current_etag".to_string() }
    }
    fn save(&self, _doc: &Doc) {}
}

async fn update_doc(req: HttpRequest, id: web::Path<String>, body: web::Bytes) -> impl Responder {
    let db = Db;
    let mut doc = db.get(id.into_inner());
    
    let want = match req.headers().get("If-Match") { // the ETag the client last saw
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("If-Match required"),
    };
    
    if want != doc.etag {                            // someone changed it first -> conflict
        return HttpResponse::PreconditionFailed().body("version conflict"); // 412
    }
    
    doc.apply(&body);
    doc.etag = new_etag();                           // bump the version
    db.save(&doc);
    
    HttpResponse::Ok()
        .insert_header(("ETag", doc.etag))
        .finish()
}
"""
    },
    "compression_the_same_negotiation_applied_11": {
        "ts": """import express, { Request, Response } from 'express';
import zlib from 'zlib';

const app = express();
const bigPayload = { data: "large amount of data" };

app.get('/data', (req: Request, res: Response) => {
    const body = JSON.stringify(bigPayload);
    
    res.setHeader('Content-Type', 'application/json');
    res.setHeader('Vary', 'Accept-Encoding'); // tell caches the body varies by encoding
    
    const acceptEncoding = req.header('Accept-Encoding') || '';
    
    if (acceptEncoding.includes('gzip')) {
        res.setHeader('Content-Encoding', 'gzip');
        zlib.gzip(body, (err, buffer) => {
            if (!err) {
                res.status(200).send(buffer);
            }
        });
        return;
    }
    
    res.status(200).send(body); // uncompressed fallback for clients that can't decode gzip
});
""",
        "rs": """use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

async fn handler(req: HttpRequest) -> impl Responder {
    let big_payload = json!({ "data": "large amount of data" });
    let body = serde_json::to_vec(&big_payload).unwrap();
    
    let accept_encoding = req.headers().get("Accept-Encoding")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
        
    // In actix-web, compression is usually handled by built-in Compress middleware.
    // To match the concept manually without external gzip crates:
    if accept_encoding.contains("gzip") {
        let compressed = vec![]; // mocked compressed bytes
        
        return HttpResponse::Ok()
            .content_type("application/json")
            .insert_header(("Vary", "Accept-Encoding")) // tell caches the body varies by encoding
            .insert_header(("Content-Encoding", "gzip"))
            .body(compressed);
    }
    
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Vary", "Accept-Encoding"))
        .body(body) // uncompressed fallback for clients that can't decode gzip
}
"""
    },
    "the_exchange_step_by_step_12": {
        "ts": """import express, { Request, Response } from 'express';
import path from 'path';

const app = express();

// express.static or res.sendFile handles Range, 206, 416 and If-Range for you,
// driven by the file's modtime and an optional ETag.
app.get('/download', (req: Request, res: Response) => {
    const filePath = path.resolve(__dirname, 'big.zip');
    res.sendFile(filePath);
});
""",
        "rs": """use actix_web::{HttpRequest, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;

// actix_files handles Range, 206, 416 and If-Range for you,
// driven by the file's modtime and an optional ETag.
async fn download(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let path: PathBuf = "big.zip".parse().unwrap();
    Ok(NamedFile::open(path)?.into_response(&req))
}
"""
    },
    "persistent_connections_keep_alive_13": {
        "ts": """import http from 'http';
import express from 'express';

const app = express();

const server = http.createServer(app);

server.timeout = 5000;          // ReadTimeout
server.keepAliveTimeout = 60000; // IdleTimeout: how long to hold an idle keep-alive conn
server.headersTimeout = 2000;    // ReadHeaderTimeout: mitigates Slowloris (slow-header attacks)

server.listen(8080, () => {
    console.log('Server listening on port 8080');
});
""",
        "rs": """use actix_web::{App, HttpServer, HttpResponse};
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
"""
    },
    "large_downloads_server_client_streaming_14": {
        "ts": """import express, { Request, Response } from 'express';
import multer from 'multer';

const app = express();

// 1. Receive a multipart upload
// up to 32 MB in memory, overflow spills to disk (handled by multer limits/disk storage)
const upload = multer({ dest: 'uploads/', limits: { fileSize: 32 * 1024 * 1024 } });

app.post('/upload', upload.single('file'), (req: Request, res: Response) => {
    const file = req.file;
    if (!file) {
        res.status(400).send('no file');
        return;
    }
    res.send(`received ${file.originalname}`);
});

// 2. Stream a response in chunks (Server-Sent Events)
app.get('/stream', (req: Request, res: Response) => {
    res.setHeader('Content-Type', 'text/event-stream');
    res.setHeader('Connection', 'keep-alive');
    
    let i = 0;
    const interval = setInterval(() => {
        res.write(`data: chunk ${i}\\n\\n`); // write() pushes each chunk to the client immediately
        i++;
        if (i >= 5) {
            clearInterval(interval);
            res.end();
        }
    }, 1000);
});
""",
        "rs": """use actix_web::{web, HttpResponse, Responder};
use futures_util::stream::StreamExt as _;
use std::time::Duration;

// 1. Receive a multipart upload (stubbed for conceptual translation)
async fn upload(mut payload: web::Payload) -> impl Responder {
    // Actix payload handles streaming data for multipart
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    HttpResponse::Ok().body("received file")
}

// 2. Stream a response in chunks (Server-Sent Events)
async fn stream() -> impl Responder {
    let stream = async_stream::stream! {
        for i in 0..5 {
            yield Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: chunk {}\\n\\n", i)));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}
"""
    }
}

for name, exts in files.items():
    ts_path = os.path.join(ts_dir, f"{name}.ts")
    rs_path = os.path.join(rs_dir, f"{name}.rs")
    with open(ts_path, 'w') as f:
        f.write(exts['ts'])
    with open(rs_path, 'w') as f:
        f.write(exts['rs'])

print("All files generated.")
