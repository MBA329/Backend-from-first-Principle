use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

async fn handler(req: HttpRequest) -> impl Responder {
    let big_payload = json!({ "data": "large amount of data" });
    let body = serde_json::to_vec(&big_payload).unwrap();
    
    let accept_encoding = req.headers().get("Accept-Encoding")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
        
    // In actix-web, compression is usually handled by built-in Compress middleware.
    // To match the concept manually without external gzip crates:
    if accept_encoding.contains("gzip") {
        let compressed = vec![]; // mocked compressed bytes
        
        return HttpResponse::Ok()
            .content_type("application/json")
            .insert_header(("Vary", "Accept-Encoding")) // tell caches the body varies by encoding
            .insert_header(("Content-Encoding", "gzip"))
            .body(compressed);
    }
    
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Vary", "Accept-Encoding"))
        .body(body) // uncompressed fallback for clients that can't decode gzip
}
