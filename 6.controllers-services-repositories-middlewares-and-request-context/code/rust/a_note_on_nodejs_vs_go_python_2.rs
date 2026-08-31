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
