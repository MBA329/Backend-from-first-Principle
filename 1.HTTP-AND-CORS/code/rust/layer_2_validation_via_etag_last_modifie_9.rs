use actix_web::{HttpRequest, HttpResponse, Responder};
use sha2::{Sha256, Digest};
use hex;

fn load_resource() -> Vec<u8> {
    b"hello world".to_vec()
}

async fn get_resource(req: HttpRequest) -> impl Responder {
    let body = load_resource();
    
    let mut hasher = Sha256::new();
    hasher.update(&body);
    let result = hasher.finalize();
    let etag = format!(""{}"", hex::encode(&result[..8])); // fingerprint of the content
    
    if let Some(if_none_match) = req.headers().get("If-None-Match") {
        if if_none_match.to_str().unwrap_or("") == etag {   // client already has this exact version
            return HttpResponse::NotModified().finish();    // 304, no body, payload saved
        }
    }
    
    HttpResponse::Ok()
        .insert_header(("ETag", etag))
        .insert_header(("Cache-Control", "max-age=10"))
        .body(body)                                         // 200 + body
}
