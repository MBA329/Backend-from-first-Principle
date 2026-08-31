use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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
