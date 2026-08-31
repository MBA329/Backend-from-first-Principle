use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use subtle::ConstantTimeEq;
use std::rc::Rc;
use redis::{AsyncCommands, Client};
use rand::RngCore;

pub struct CheckCsrf;

impl<S, B> Transform<S, ServiceRequest> for CheckCsrf
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckCsrfMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckCsrfMiddleware { service: Rc::new(service) })
    }
}

pub struct CheckCsrfMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CheckCsrfMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // Double-submit: compare the CSRF cookie against the header.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().as_str();
        if method == "GET" || method == "HEAD" {
            let srv = self.service.clone();
            return Box::pin(async move { srv.call(req).await });
        }

        let cookie = req.cookie("csrf");
        let header = req.headers().get("X-CSRF-Token");

        let valid = match (cookie, header) {
            (Some(c), Some(h)) => {
                let c_bytes = c.value().as_bytes();
                let h_bytes = h.as_bytes();
                if c_bytes.len() == h_bytes.len() {
                    c_bytes.ct_eq(h_bytes).into()
                } else {
                    false
                }
            },
            _ => false,
        };

        if !valid {
            return Box::pin(async move {
                Err(actix_web::error::ErrorForbidden("forbidden"))
            });
        }

        let srv = self.service.clone();
        Box::pin(async move { srv.call(req).await })
    }
}

pub fn new_session_id() -> String {
    let mut b = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut b); // cryptographically random
    hex::encode(b)
}

// Regenerate the session ID right after a successful login.
pub async fn regenerate_session(rdb: &Client, old_sid: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut con = rdb.get_async_connection().await?;
    
    let raw: Option<String> = con.get(format!("sess:{}", old_sid)).await?;
    let _: () = con.del(format!("sess:{}", old_sid)).await?; // kill the pre-login ID
    
    let new_sid = new_session_id(); // brand-new value
    
    if let Some(r) = raw {
        let _: () = con.set_ex(format!("sess:{}", new_sid), r, 15 * 60).await?;
    }
    
    Ok(new_sid)
}
