use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListQuery {
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<u32>,
    pub limit: Option<u32>,
}

// GET /v1/organizations?status=active&sortBy=name&sortOrder=ascending&page=1&limit=10
pub async fn list_organizations(query: web::Query<ListQuery>) -> impl Responder {
    // --- sane defaults: never crash if the client omits params ---
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let sort_by = query.sort_by.clone().unwrap_or_else(|| "createdAt".to_string());
    let sort_order = query.sort_order.clone().unwrap_or_else(|| "descending".to_string());

    let mut filters = std::collections::HashMap::new();
    if let Some(ref s) = query.status {
        filters.insert("status".to_string(), s.clone()); // ?status=active
    }

    let (rows, total) = store_query(filters, &sort_by, &sort_order, page, limit).await;
    let total_pages = (total + limit - 1) / limit; // ceil division

    HttpResponse::Ok().json(serde_json::json!({
        "data": rows,
        "total": total,
        "page": page,
        "totalPages": total_pages,
    }))
}

// Mock
async fn store_query(_filters: std::collections::HashMap<String, String>, _sort_by: &str, _sort_order: &str, _page: u32, _limit: u32) -> (Vec<serde_json::Value>, u32) {
    (vec![], 0)
}
