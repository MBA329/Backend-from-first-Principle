// Rust, Full text search handler using PostgreSQL
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub rank: f64,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
}

async fn search_handler(pool: web::Data<PgPool>, info: web::Query<SearchQuery>) -> impl Responder {
    let query = match &info.q {
        Some(q) if !q.is_empty() => q,
        _ => return HttpResponse::BadRequest().body("missing query param q"),
    };

    // plainto_tsquery converts plain text safely (no special chars needed)
    // websearch_to_tsquery also supports AND/OR/-term syntax
    let sql = r#"
        SELECT id, name, description,
               ts_rank(search_vec, plainto_tsquery('english', $1)) AS rank
        FROM   products
        WHERE  search_vec @@ plainto_tsquery('english', $1)
        ORDER  BY rank DESC
        LIMIT  20
    "#;

    match sqlx::query_as::<_, Product>(sql).bind(query).fetch_all(pool.get_ref()).await {
        Ok(results) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(format!("found {} results\n", results.len()))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://...").await.unwrap();
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/search", web::get().to(search_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
