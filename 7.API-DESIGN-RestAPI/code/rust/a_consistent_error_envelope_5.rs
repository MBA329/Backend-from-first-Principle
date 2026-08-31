use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub issue: String,
}

pub fn write_error(status: actix_web::http::StatusCode, code: &str, msg: &str, details: Vec<FieldError>, request_id: &str) -> HttpResponse {
    let body = serde_json::json!({
        "error": {
            "code": code,
            "message": msg,
            "details": details,
            "requestId": request_id,
        }
    });
    
    HttpResponse::build(status).json(body) // SAME shape for every error in the whole API
}

// usage example
pub fn example_usage() -> HttpResponse {
    write_error(
        actix_web::http::StatusCode::UNPROCESSABLE_ENTITY, 
        "validation_failed", 
        "Some fields are invalid.",
        vec![
            FieldError { field: "email".to_string(), issue: "must be a valid email".to_string() },
            FieldError { field: "age".to_string(), issue: "must be >= 0".to_string() },
        ],
        "req_12345"
    )
}
