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
