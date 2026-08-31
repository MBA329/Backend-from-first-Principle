import os

def write_file(path, content):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write(content.strip() + "\n")

# CHAPTER 5
ch5_ts = "5. Validations and transformations for backend engineers/code/typescript"
ch5_rs = "5. Validations and transformations for backend engineers/code/rust"

write_file(f"{ch5_ts}/example_1.ts", """
import { z } from 'zod';
import { Request, Response } from 'express';

// CreateBook is the SCHEMA. Each annotation = one rule
const CreateBookSchema = z.object({
  name: z.string().min(5).max(100)
});
type CreateBook = z.infer<typeof CreateBookSchema>;

// runPipeline decodes then validates; returns 400-ready messages.
export function runPipeline(req: Request, res: Response): CreateBook | null {
  try {
    // constructing the model runs all three layers
    // in order: existence -> type -> constraint
    return CreateBookSchema.parse(req.body);
  } catch (e) {
    if (e instanceof z.ZodError) {
      const messages = e.errors.map(err => `${err.path.join('.')}: ${err.message}`);
      res.status(400).json({ detail: messages });
      return null;
    }
    throw e;
  }
}
""")

write_file(f"{ch5_rs}/example_1.rs", """
use actix_web::{web, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::fmt;

// CreateBook is the SCHEMA. Each annotation = one rule:
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 5, max = 100))]
    pub name: String,
}

// runPipeline decodes then validates; returns 400-ready messages.
pub async fn run_pipeline(body: web::Json<CreateBook>) -> Result<CreateBook, actix_web::Error> {
    let create_book = body.into_inner();
    
    // 2. EXISTENCE + CONSTRAINT checks in one pass.
    if let Err(e) = create_book.validate() {
        // reshape errors into clean strings
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }
    Ok(create_book)
}
""")

write_file(f"{ch5_ts}/example_2.ts", """
import { z } from 'zod';

export const TypePayloadSchema = z.object({
  stringField: z.string(),
  numberField: z.number(),
  // recursive: EVERY element a string
  arrayField: z.array(z.string()),
  boolField: z.boolean(),
});
export type TypePayload = z.infer<typeof TypePayloadSchema>;

// Strict, explicit type errors are caught by Zod during parsing.
// Like pydantic, Zod catches type mismatches.
""")

write_file(f"{ch5_rs}/example_2.rs", """
use serde::Deserialize;

// *Option<bool> so we can tell "false" apart from "missing".
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)] // reject stray keys
pub struct TypePayload {
    pub string_field: String,
    pub number_field: f64,
    pub array_field: Vec<String>,
    pub bool_field: Option<bool>,
}

// serde_json enforces the BASE types: a string for
// number_field / array_field / bool_field fails to deserialize.
""")

write_file(f"{ch5_ts}/example_3.ts", """
import { z } from 'zod';

// optional + then 7-15 digits (country code + national no.)
const phoneRe = /^\+?[0-9]{7,15}$/;

export const ContactSchema = z.object({
  email: z.string().email(), // local @ domain.tld
  phone: z.string().regex(phoneRe, "invalid phone number format"),
  date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "only accepts YYYY-MM-DD") // YYYY-MM-DD
});
export type Contact = z.infer<typeof ContactSchema>;
""")

write_file(f"{ch5_rs}/example_3.rs", """
use serde::Deserialize;
use validator::Validate;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref PHONE_RE: Regex = Regex::new(r"^\+?[0-9]{7,15}$").unwrap();
}

#[derive(Debug, Deserialize, Validate)]
pub struct Contact {
    #[validate(email)]
    pub email: String, // local @ domain.tld
    #[validate(regex(path = "PHONE_RE", message = "invalid phone number format"))]
    pub phone: String, // checked below
    // custom validation for YYYY-MM-DD
    pub date: String, 
}
""")

write_file(f"{ch5_ts}/example_4.ts", """
import { z } from 'zod';

export const ProfileSchema = z.object({
  dateOfBirth: z.string().refine((val) => {
    const dob = new Date(val);
    if (isNaN(dob.getTime())) return false;
    if (dob > new Date()) return false;
    return true;
  }, { message: "date of birth cannot be in the future" }),
  age: z.number().min(1).max(120) // gte/lte cover the "430 is impossible" semantic bound
});
export type Profile = z.infer<typeof ProfileSchema>;
""")

write_file(f"{ch5_rs}/example_4.rs", """
use serde::Deserialize;
use validator::{Validate, ValidationError};
use chrono::{NaiveDate, Utc};

#[derive(Debug, Deserialize, Validate)]
pub struct Profile {
    #[validate(custom = "validate_not_in_future")]
    pub date_of_birth: String,
    // gte/lte cover the "430 is impossible" semantic bound
    #[validate(range(min = 1, max = 120))]
    pub age: i32,
}

// Type & syntax can't express "not in the future" , 
// semantics need real logic against the real clock.
fn validate_not_in_future(date: &str) -> Result<(), ValidationError> {
    let parsed = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| ValidationError::new("invalid date"))?;
    if parsed > Utc::now().naive_utc().date() {
        return Err(ValidationError::new("date of birth cannot be in the future"));
    }
    Ok(())
}
""")

write_file(f"{ch5_ts}/example_5.ts", """
import { z } from 'zod';

export const SignupSchema = z.object({
  password: z.string().min(8),
  passwordConfirmation: z.string(),
  married: z.boolean(),
  partner: z.string().optional()
}).refine((data) => data.password === data.passwordConfirmation, {
  message: "passwords don't match",
  path: ["passwordConfirmation"]
}).refine((data) => {
  if (data.married && !data.partner) return false;
  return true;
}, {
  message: "partner name is required when married is true",
  path: ["partner"]
});
export type Signup = z.infer<typeof SignupSchema>;
""")

write_file(f"{ch5_rs}/example_5.rs", """
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "cross_field_rules", skip_on_field_errors = false))]
pub struct Signup {
    #[validate(length(min = 8))]
    pub password: String,
    pub password_confirmation: String,
    pub married: bool,
    pub partner: Option<String>,
}

fn cross_field_rules(s: &Signup) -> Result<(), ValidationError> {
    if s.password != s.password_confirmation {
        return Err(ValidationError::new("passwords don't match"));
    }
    if s.married && s.partner.is_none() {
        return Err(ValidationError::new("partner name is required when married is true"));
    }
    Ok(())
}
""")

write_file(f"{ch5_ts}/example_6.ts", """
import { z } from 'zod';

export const PaginationSchema = z.object({
  // Zod coercing the incoming string "2" into int 2 (transform), 
  // THEN enforces gt/lt (validate)
  page: z.coerce.number().int().min(1).max(499),
  limit: z.coerce.number().int().min(1).max(9999)
});
export type Pagination = z.infer<typeof PaginationSchema>;

export function parsePagination(q: any): Pagination {
    return PaginationSchema.parse(q);
}
""")

write_file(f"{ch5_rs}/example_6.rs", """
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Pagination {
    // Serde automatically CASTS (transform) string from query param 
    // into int before we can VALIDATE the numbers.
    #[validate(range(min = 1, max = 499))]
    pub page: i32,
    #[validate(range(min = 1, max = 9999))]
    pub limit: i32,
}
""")

write_file(f"{ch5_ts}/example_7.ts", """
import { z } from 'zod';

export const ContactSchema = z.object({
  email: z.string().email().transform(v => v.trim().toLowerCase()),
  phone: z.string().transform(v => {
    v = v.trim();
    return v.startsWith('+') ? v : '+' + v;
  })
});
export type Contact = z.infer<typeof ContactSchema>;
""")

write_file(f"{ch5_rs}/example_7.rs", """
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
}

impl Contact {
    // normalize runs AFTER validation passes and BEFORE
    // the data is handed to the service layer.
    pub fn normalize(&mut self) {
        self.email = self.email.trim().to_lowercase();
        
        let trimmed_phone = self.phone.trim();
        if !trimmed_phone.starts_with('+') {
            self.phone = format!("+{}", trimmed_phone);
        } else {
            self.phone = trimmed_phone.to_string();
        }
    }
}
""")

write_file(f"{ch5_ts}/example_8.ts", """
import express, { Request, Response } from 'express';
import { z } from 'zod';

const app = express();
app.use(express.json());

// ---- schema (the gate) ----
const CreateBookSchema = z.object({
  name: z.string().min(5).max(100).transform(v => v.trim())
});
type CreateBook = z.infer<typeof CreateBookSchema>;

interface Book {
  id: number;
  name: string;
}

// ---- service + repository (sketched) ----
async function createBook(name: string): Promise<Book> {
  // service logic ... repository INSERT ... then:
  return { id: 1, name };
}

// ---- CONTROLLER ----
app.post("/api/books", async (req: Request, res: Response) => {
  try {
    // === GATE ,  runs before ANY business logic ===
    const body = CreateBookSchema.parse(req.body);

    // === only now: business logic (service -> repo) ===
    const book = await createBook(body.name);
    res.status(201).json(book);
  } catch (error) {
    if (error instanceof z.ZodError) {
      res.status(400).json({ error: error.errors });
    } else {
      res.status(500).json({ error: "could not create book" });
    }
  }
});

app.listen(8080);
""")

write_file(f"{ch5_rs}/example_8.rs", """
use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ---- schema (the gate) ----
#[derive(Debug, Deserialize, Validate)]
pub struct CreateBook {
    #[validate(length(min = 5, max = 100))]
    pub name: String,
}

#[derive(Serialize)]
pub struct Book {
    pub id: i32,
    pub name: String,
}

// ---- service + repository (sketched) ----
async fn create_book(name: String) -> Result<Book, String> {
    // service logic ... repository INSERT ... then:
    Ok(Book { id: 1, name })
}

// ---- CONTROLLER ----
async fn create_book_endpoint(body: web::Json<CreateBook>) -> impl Responder {
    let mut create_req = body.into_inner();
    create_req.name = create_req.name.trim().to_string(); // transform
    
    // === GATE ,  runs before ANY business logic ===
    if let Err(err) = create_req.validate() {
        return HttpResponse::BadRequest().json(err.to_string());
    }

    // === only now: business logic (service -> repo) ===
    match create_book(create_req.name).await {
        Ok(book) => HttpResponse::Created().json(book),
        Err(_) => HttpResponse::InternalServerError().json("could not create book"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/api/books", web::post().to(create_book_endpoint))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
""")

# CHAPTER 6
ch6_ts = "6.controllers-services-repositories-middlewares-and-request-context/code/typescript"
ch6_rs = "6.controllers-services-repositories-middlewares-and-request-context/code/rust"

write_file(f"{ch6_ts}/a_note_on_nodejs_vs_go_python_1.ts", """
import { Request, Response } from 'express';

interface CreateBookRequest {
    title: string;
    author: string;
}

export function createBookHandler(req: Request, res: Response) {
    // Step 1 + 2: extract body and deserialize into a native class/dict.
    const payload = req.body as CreateBookRequest;
    if (!payload || !payload.title) {
        // Deserialization failed -> malformed payload.
        res.status(400).json({ error: "invalid request body" });
        return; // terminate the request here; do not proceed.
    }
    // ... validation, transformation, delegation follow ...
}
""")

write_file(f"{ch6_rs}/a_note_on_nodejs_vs_go_python_1.rs", """
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

// CreateBookRequest is our native format ,  the bind target.
#[derive(Deserialize)]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
}

pub async fn create_book_handler(req: web::Json<CreateBookRequest>) -> impl Responder {
    // Step 1 + 2: extract body and deserialize into struct handled by actix.
    // If it fails, actix automatically returns 400 Bad Request.
    let _payload = req.into_inner();
    
    // ... validation, transformation, delegation follow ...
    HttpResponse::Ok().finish()
}
""")

write_file(f"{ch6_ts}/a_note_on_nodejs_vs_go_python_2.ts", """
import { Request, Response } from 'express';

const bookService = { listBooks: async (sort: string) => [] }; // sketch

export async function listBooksHandler(req: Request, res: Response) {
    let sort = req.query.sort as string;

    // VALIDATION: if present, must be an allowed value.
    if (sort && sort !== "name" && sort !== "date") {
        res.status(400).json({ error: "sort must be 'name' or 'date'" });
        return;
    }

    // TRANSFORMATION: optional param -> inject a default.
    if (!sort) {
        sort = "date";
    }

    try {
        const books = await bookService.listBooks(sort); // delegate
        res.status(200).json(books); // array of books
    } catch (e) {
        res.status(500).json({ error: "could not fetch books" });
    }
}
""")

write_file(f"{ch6_rs}/a_note_on_nodejs_vs_go_python_2.rs", """
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryInfo {
    sort: Option<String>,
}

// Sketch
struct BookService;
impl BookService {
    async fn list_books(&self, sort: &str) -> Result<Vec<String>, ()> { Ok(vec![]) }
}

pub async fn list_books_handler(info: web::Query<QueryInfo>, service: web::Data<BookService>) -> impl Responder {
    let mut sort = info.sort.clone().unwrap_or_default();

    // VALIDATION: if present, it must be one of the allowed values.
    if !sort.is_empty() && sort != "name" && sort != "date" {
        return HttpResponse::BadRequest().body("sort must be 'name' or 'date'");
    }

    // TRANSFORMATION: query params are optional -> inject a default.
    if sort.is_empty() {
        sort = "date".to_string(); // downstream layers never see an empty value
    }

    match service.list_books(&sort).await {
        Ok(books) => HttpResponse::Ok().json(books),
        Err(_) => HttpResponse::InternalServerError().body("could not fetch books"),
    }
}
""")

write_file(f"{ch6_ts}/06the_service_layer_3.ts", """
export class BookService {
    constructor(private repo: any, private mailer: any) {}

    // No request, no response, no status codes ,  just logic.
    async listBooks(sort: string) {
        // Orchestration: ask the repository for what it needs.
        const books = await this.repo.findAllBooks(sort);
        // Could merge other repo calls, enrich, notify, etc.
        return books;
    }

    // A purely-logic service that never touches the DB:
    async notifyOwner(email: string) {
        return this.mailer.send(email, "Your book was added");
    }
}
""")

write_file(f"{ch6_rs}/06the_service_layer_3.rs", """
pub struct BookService {
    repo: Box<dyn BookRepoTrait>,
    mailer: Box<dyn MailerTrait>,
}

impl BookService {
    // Notice: no request, no response, no status codes.
    // You cannot tell from this signature that it serves an API.
    pub async fn list_books(&self, sort: &str) -> Result<Vec<Book>, Box<dyn std::error::Error>> {
        // Orchestration: call the repository for the data it needs.
        let books = self.repo.find_all_books(sort).await?;
        // Could also: enrich, merge other repo calls, send notifications...
        Ok(books)
    }

    // A service that needs no database at all is perfectly valid:
    pub async fn notify_owner(&self, email: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.mailer.send(email, "Your book was added").await
    }
}

// Sketches
pub trait BookRepoTrait {
    fn find_all_books(&self, sort: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<Book>, Box<dyn std::error::Error>>>>>;
}
pub trait MailerTrait {
    fn send(&self, email: &str, msg: &str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>>>;
}
pub struct Book {}
""")

write_file(f"{ch6_ts}/07the_repository_layer_4.ts", """
export class BookRepo {
    constructor(private db: any) {}

    // ONE method, ONE result shape: all books, sorted.
    async findAllBooks(sort: string) {
        // Build the query from data passed down by the service.
        const query = `SELECT id, title, author FROM books ORDER BY ${sort}`;
        const [rows] = await this.db.execute(query);
        return rows;
    }

    // SEPARATE method for one book. No optional toggle parameter.
    async findBookById(bookId: number) {
        const query = "SELECT id, title, author FROM books WHERE id = ?";
        const [rows] = await this.db.execute(query, [bookId]);
        return rows[0];
    }
}
""")

write_file(f"{ch6_rs}/07the_repository_layer_4.rs", """
use sqlx::PgPool;

pub struct BookRepo {
    db: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

impl BookRepo {
    // ONE method, ONE shape of result: all books, sorted.
    pub async fn find_all_books(&self, sort: &str) -> Result<Vec<Book>, sqlx::Error> {
        // Build the query from the data the service handed down.
        let query = format!("SELECT id, title, author FROM books ORDER BY {}", sort);
        let books = sqlx::query_as::<_, Book>(&query)
            .fetch_all(&self.db)
            .await?;
        Ok(books)
    }

    // A SEPARATE method for the single-book case. No optional toggles.
    pub async fn find_book_by_id(&self, id: i32) -> Result<Book, sqlx::Error> {
        let book = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books WHERE id = $1")
            .bind(id)
            .fetch_one(&self.db)
            .await?;
        Ok(book)
    }
}
""")

write_file(f"{ch6_ts}/10the_next_function_5.ts", """
import { Request, Response, NextFunction } from 'express';

// An Express middleware receives the request, response, and `next`
// function that invokes the rest of the chain.
export function loggingMiddleware(req: Request, res: Response, next: NextFunction) {
    console.log(`${req.method} ${req.path}`); // do work

    // EARLY EXIT example (short-circuit, never calls next):
    if (req.headers["x-blocked"] === "yes") {
        res.status(403).send("forbidden");
        return; // stops here
    }

    next(); // === next(): continue the chain ===
}
""")

write_file(f"{ch6_rs}/10the_next_function_5.rs", """
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_web::dev::Transform;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::dev::Service;

// In Rust (Actix-web), middleware wraps the next service. Calling Service::call
// is the equivalent of next(): pass execution along the chain.
pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("{} {}", req.method(), req.path()); // do work

        // EARLY EXIT example (short-circuit, never calls next):
        if req.headers().get("X-Blocked").map(|h| h.to_str().unwrap_or("")) == Some("yes") {
            let (http_req, _payload) = req.into_parts();
            let res = HttpResponse::Forbidden().body("forbidden");
            // Must return matched generic B type
            // (skipped exact response mapping for sketch simplicity)
        }

        // next(): continue the chain
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
""")

write_file(f"{ch6_ts}/12common_middlewares_6.ts", """
import { Request, Response, NextFunction } from 'express';

const ALLOWED_ORIGIN = "https://app.example.com";

export function corsMiddleware(req: Request, res: Response, next: NextFunction) {
    const origin = req.headers.origin; // runtime gives us this
    if (origin === ALLOWED_ORIGIN) {
        res.setHeader("Access-Control-Allow-Origin", origin);
    }
    next(); // pass along; browser blocks if header absent
}

export function authMiddleware(req: Request, res: Response, next: NextFunction) {
    const token = req.headers.authorization;
    try {
        const { userId, role } = verifyToken(token); // Sketch
        // SUCCESS: stash identity in the request locals, then continue.
        res.locals.userId = userId;
        res.locals.role = role;
        next();
    } catch (e) {
        res.status(401).send("unauthorized"); // stop
    }
}

function verifyToken(token: any): any { return {}; } // Sketch
""")

write_file(f"{ch6_rs}/12common_middlewares_6.rs", """
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_web::dev::Transform;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::dev::Service;

const ALLOWED_ORIGIN: &str = "https://app.example.com";

// (CORS is typically handled by actix_cors::Cors, but here is a middleware conceptual sketch)

// Auth Middleware concept
pub struct AuthMiddleware<S> { service: S }

#[derive(Clone)]
pub struct AuthCtx {
    pub user_id: i32,
    pub role: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        
        // Sketch verify_token
        let is_valid = true; 
        
        if !is_valid {
            // return 401
        }
        
        // SUCCESS: stash identity in the request context, then continue.
        req.extensions_mut().insert(AuthCtx { user_id: 1, role: "user".to_string() });

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
""")

write_file(f"{ch6_ts}/14use_case_passing_authentication_data_7.ts", """
import { Request, Response } from 'express';

const bookService = { create: (req: any, id: any) => ({}) }; // sketch

export function createBookHandler(req: Request, res: Response) {
    const payload = req.body;

    // Read the trusted user id FROM THE CONTEXT, not from the body.
    // The auth middleware put it there after verifying the token.
    const userId = res.locals.userId;
    const role = res.locals.role;

    if (role !== "admin" && role !== "user") {
        res.status(403).send("forbidden");
        return;
    }

    // Persist with the SERVER-VERIFIED owner id ,  never the client's.
    const book = bookService.create(payload, userId);
    res.status(201).json(book);
}
""")

write_file(f"{ch6_rs}/14use_case_passing_authentication_data_7.rs", """
use actix_web::{web, HttpResponse, Responder, HttpMessage, HttpRequest};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateBookRequest {
    title: String,
    author: String,
}

#[derive(Clone)]
pub struct AuthCtx {
    pub user_id: i32,
    pub role: String,
}

// Sketch
struct BookService;
impl BookService {
    fn create(&self, req: CreateBookRequest, user_id: i32) -> String { "".into() }
}

pub async fn create_book_handler(
    req: HttpRequest,
    body: web::Json<CreateBookRequest>,
    service: web::Data<BookService>
) -> impl Responder {
    // Read the trusted user ID FROM THE CONTEXT, not from req.
    // The auth middleware put it there after verifying the token.
    let extensions = req.extensions();
    let auth_ctx = extensions.get::<AuthCtx>().expect("Auth missing");

    if auth_ctx.role != "admin" && auth_ctx.role != "user" {
        return HttpResponse::Forbidden().body("forbidden");
    }

    // Persist with the SERVER-VERIFIED owner id ,  never the client's.
    let book = service.create(body.into_inner(), auth_ctx.user_id);
    HttpResponse::Created().json(book) // 201
}
""")

write_file(f"{ch6_ts}/cancellation_abort_signals_deadlines_8.ts", """
import { Request, Response, NextFunction } from 'express';
import { v4 as uuidv4 } from 'uuid';

export function requestIdMiddleware(req: Request, res: Response, next: NextFunction) {
    const rid = uuidv4(); // one unique id for this request
    res.locals.requestId = rid;
    console.log(`[${rid}] ${req.method} ${req.path}`);
    
    // Express res object headers
    res.setHeader("X-Request-ID", rid); // echo it back / forward it
    
    next();
}
""")

write_file(f"{ch6_rs}/cancellation_abort_signals_deadlines_8.rs", """
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::dev::Service;
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

pub struct RequestIdMiddleware<S> { service: S }

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let id = Uuid::new_v4().to_string(); // one unique id for this request
        req.extensions_mut().insert(id.clone());
        println!("[{}] {} {}", id, req.method(), req.path());
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-request-id"),
                actix_web::http::header::HeaderValue::from_str(&id).unwrap(),
            );
            Ok(res)
        })
    }
}
""")
print("Generation complete")
