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
