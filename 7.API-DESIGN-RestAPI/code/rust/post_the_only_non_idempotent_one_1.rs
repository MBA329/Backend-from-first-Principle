use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub etag: String,
    // other fields...
}

pub async fn update_organization(
    req: HttpRequest,
    id: web::Path<String>,
    patch: web::Json<serde_json::Value>,
) -> impl Responder {
    let org = store_get(&id).await;
    
    if org.is_none() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "error": { "message": "organization not found" }
        }));
    }
    let org = org.unwrap();

    // optimistic concurrency: reject stale writes
    if let Some(m) = req.headers().get("If-Match") {
        if let Ok(match_str) = m.to_str() {
            if match_str != org.etag {
                return HttpResponse::PreconditionFailed().json(serde_json::json!({
                    "error": { "message": "resource changed; re-fetch and retry" }
                }));
            }
        }
    }

    let updated = store_patch(&id, &patch).await; // merge, don't replace
    
    HttpResponse::Ok()
        .insert_header(("ETag", updated.etag.clone()))
        .json(updated) // 200 + updated entity
}

// Mocks
async fn store_get(id: &str) -> Option<Organization> {
    Some(Organization { id: id.to_string(), etag: "123".to_string() })
}
async fn store_patch(id: &str, _patch: &serde_json::Value) -> Organization {
    Organization { id: id.to_string(), etag: "124".to_string() }
}
