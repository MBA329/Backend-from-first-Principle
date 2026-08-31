use actix_web::{web, HttpRequest, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

// key -> String result. Use Redis with a TTL in production.
struct AppState {
    seen: Mutex<HashMap<String, String>>,
}

fn charge(_req: &HttpRequest) -> String {
    "payment_success".to_string()
}

async fn create_payment(req: HttpRequest, data: web::Data<AppState>) -> impl Responder {
    let key_header = req.headers().get("Idempotency-Key");
    
    let key = match key_header {
        Some(k) => match k.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return HttpResponse::BadRequest().body("invalid Idempotency-Key"),
        },
        None => return HttpResponse::BadRequest().body("missing Idempotency-Key"),
    };
    
    let mut seen = data.seen.lock().unwrap();
    if let Some(cached) = seen.get(&key) {    // replay: return the stored result
        return HttpResponse::Ok().body(cached.clone());
    }
    
    let result = charge(&req);                // the real, non-idempotent work
    seen.insert(key, result.clone());         // remember it BEFORE responding
    
    HttpResponse::Created().body(result)
}
