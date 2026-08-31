use actix_web::{web, HttpRequest, HttpResponse, Responder};

// Permanent move that must keep the method/body -> 308
async fn old_route(id: web::Path<String>) -> impl Responder {
    let target = format!("/person/{}", id.into_inner());
    HttpResponse::PermanentRedirect() // 308
        .insert_header(("Location", target))
        .finish()
}

fn save(_req: &HttpRequest) -> String {
    "new_id".to_string()
}

// Post/Redirect/Get -> 303 so a browser refresh won't re-POST the form
async fn submit_form(req: HttpRequest) -> impl Responder {
    let id = save(&req);
    let target = format!("/results/{}", id);
    HttpResponse::SeeOther() // 303 (forces GET)
        .insert_header(("Location", target))
        .finish()
}
