use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: String,
}

impl User {
    fn visible_to(&self, req: &HttpRequest) -> bool {
        if let Some(auth) = req.headers().get("Authorization") {
            return auth.to_str().unwrap_or("") == "user";
        }
        false
    }
}

async fn get_user(req: HttpRequest, id: web::Path<String>) -> impl Responder {
    if req.headers().get("Authorization").is_none() {
        return HttpResponse::Unauthorized().body("login required"); // 401: who are you?
    }
    
    let id_val = id.into_inner();
    
    // simulated db.Find
    if id_val != "1" {
        return HttpResponse::NotFound().body("no such user");       // 404
    }
    
    let user = User { id: id_val };
    
    if !user.visible_to(&req) {
        return HttpResponse::Forbidden().body("forbidden");         // 403: not allowed
    }
    
    HttpResponse::Ok().json(user)                                   // 200
}
