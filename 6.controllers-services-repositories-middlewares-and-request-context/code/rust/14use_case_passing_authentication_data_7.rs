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
