use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub fn register_org_routes(cfg: &mut web::ServiceConfig) {
    // list + create share the collection URL, split by method
    cfg.route("/v1/organizations", web::get().to(list_organizations));
    cfg.route("/v1/organizations", web::post().to(create_organization));

    // get-one / update / delete share /:id, split by method
    cfg.route("/v1/organizations/{id}", web::get().to(get_organization));
    cfg.route("/v1/organizations/{id}", web::patch().to(update_organization));
    cfg.route("/v1/organizations/{id}", web::delete().to(delete_organization));

    // custom action: verb at the end of a specific resource
    cfg.route("/v1/organizations/{id}/archive", web::post().to(archive_organization));
}

#[derive(Deserialize)]
pub struct CreateOrgRequest {
    pub name: String,
    pub status: Option<String>,
    pub description: String,
}

#[derive(Serialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub status: String,
    pub description: String,
}

pub async fn create_organization(body: web::Json<CreateOrgRequest>) -> impl Responder {
    let status = body.status.clone().unwrap_or_else(|| "active".to_string()); // sane default, don't force the client to send the obvious
    
    let org = store_insert(&body.name, &status, &body.description).await; // id, createdAt set server-side
    
    HttpResponse::Created().json(org) // 201 Created + the new entity
}

pub async fn delete_organization(path: web::Path<String>) -> impl Responder {
    store_delete(&path).await;
    HttpResponse::NoContent().finish() // No Content
}

pub async fn archive_organization(path: web::Path<String>) -> impl Responder {
    let org = store_archive(&path).await; // flips status + cascades: projects, tasks, emails...
    HttpResponse::Ok().json(org) // custom action -> 200, NOT 201
}

// Mocks
async fn list_organizations() -> impl Responder { HttpResponse::Ok().finish() }
async fn get_organization() -> impl Responder { HttpResponse::Ok().finish() }
async fn update_organization() -> impl Responder { HttpResponse::Ok().finish() }
async fn store_insert(name: &str, status: &str, description: &str) -> Organization {
    Organization { id: "org_1".to_string(), name: name.to_string(), status: status.to_string(), description: description.to_string() }
}
async fn store_delete(_id: &str) {}
async fn store_archive(id: &str) -> Organization {
    Organization { id: id.to_string(), name: "Org".to_string(), status: "archived".to_string(), description: "".to_string() }
}
