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
