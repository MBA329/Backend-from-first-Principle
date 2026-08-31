use actix_web::{web, cookie::{Cookie, SameSite}, dev::{Service, ServiceRequest, ServiceResponse, Transform}, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer, Responder};
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};
use std::collections::HashMap;
use std::sync::Mutex;

struct AppState {
    sessions: Mutex<HashMap<String, String>>, // server-side session store (use Redis)
}

fn new_session_id() -> String {
    "random_uuid_here".to_string() // mock
}

// Set a secure session cookie after login (stateful)
async fn login(data: web::Data<AppState>) -> impl Responder {
    let sid = new_session_id();
    let user_id = "user123".to_string();
    
    let mut sessions = data.sessions.lock().unwrap();
    sessions.insert(sid.clone(), user_id);
    
    let cookie = Cookie::build("session", sid)
        .http_only(true)                    // JS can't read it
        .secure(true)                       // HTTPS only
        .same_site(SameSite::Strict)        // CSRF defense
        .max_age(actix_web::cookie::time::Duration::seconds(3600))
        .path("/")
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .body("logged in")
}

fn valid_jwt(token: &str) -> bool {
    token == "valid_token"
}

// Bearer-token auth middleware (stateless) - Using Actix Web Middleware Pattern
pub struct RequireToken;

impl<S, B> Transform<S, ServiceRequest> for RequireToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequireTokenMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireTokenMiddleware { service: Rc::new(service) })
    }
}

pub struct RequireTokenMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireTokenMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth = req.headers().get("Authorization").and_then(|h| h.to_str().ok());
        
        let valid = match auth {
            Some(auth_str) if auth_str.starts_with("Bearer ") => {
                valid_jwt(&auth_str[7..])
            }
            _ => false,
        };

        if !valid {
            let (req, _pl) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .append_header(("WWW-Authenticate", "Bearer"))
                .body("unauthorized")
                .map_into_right_body();
            return Box::pin(async { Ok(ServiceResponse::new(req, response)) });
        }

        let svc = self.service.clone();
        Box::pin(async move {
            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
