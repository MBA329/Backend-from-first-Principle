use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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
