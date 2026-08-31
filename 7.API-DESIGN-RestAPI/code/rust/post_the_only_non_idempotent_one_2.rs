use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub amount: i32,
}

#[derive(Serialize, Clone)]
pub struct PaymentResponse {
    pub id: String,
    pub amount: i32,
}

#[derive(Clone)]
pub struct PriorResponse {
    pub status: u16,
    pub body: serde_json::Value,
}

pub async fn create_payment(
    req: HttpRequest,
    body: web::Json<PaymentRequest>,
) -> impl Responder {
    // client-generated UUID
    let key = match req.headers().get("Idempotency-Key") {
        Some(k) => k.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error": "Idempotency-Key header required"})),
    };

    // already processed this exact request? return the SAME result, don't re-charge
    if let Some(prior) = idem_store_get(key).await {
        return HttpResponse::build(actix_web::http::StatusCode::from_u16(prior.status).unwrap())
            .json(prior.body);
    }

    let payment = charge(body.amount).await; // the real, non-idempotent side effect
    
    idem_store_save(key, 201, serde_json::to_value(&payment).unwrap()).await; // remember it, keyed by the idempotency key
    
    HttpResponse::Created().json(payment) // 201 Created
}

// Mocks
async fn idem_store_get(_key: &str) -> Option<PriorResponse> { None }
async fn idem_store_save(_key: &str, _status: u16, _body: serde_json::Value) {}
async fn charge(amount: i32) -> PaymentResponse {
    PaymentResponse { id: "pay_123".to_string(), amount }
}
