use actix_web::{web, HttpRequest, HttpResponse, Responder};

fn new_etag() -> String {
    "new_random_etag".to_string()
}

struct Doc {
    id: String,
    etag: String,
}

impl Doc {
    fn apply(&mut self, _body: &web::Bytes) {}
}

struct Db;
impl Db {
    fn get(&self, id: String) -> Doc {
        Doc { id, etag: "current_etag".to_string() }
    }
    fn save(&self, _doc: &Doc) {}
}

async fn update_doc(req: HttpRequest, id: web::Path<String>, body: web::Bytes) -> impl Responder {
    let db = Db;
    let mut doc = db.get(id.into_inner());
    
    let want = match req.headers().get("If-Match") { // the ETag the client last saw
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::BadRequest().body("If-Match required"),
    };
    
    if want != doc.etag {                            // someone changed it first -> conflict
        return HttpResponse::PreconditionFailed().body("version conflict"); // 412
    }
    
    doc.apply(&body);
    doc.etag = new_etag();                           // bump the version
    db.save(&doc);
    
    HttpResponse::Ok()
        .insert_header(("ETag", doc.etag))
        .finish()
}
