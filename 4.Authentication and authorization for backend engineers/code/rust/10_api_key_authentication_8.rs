use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use sha2::{Sha256, Digest};
use subtle::ConstantTimeEq;

#[derive(Clone)]
pub struct ClientInfo {
    pub id: String,
}

// Store only the HASH of issued keys (like passwords).
// On each call, hash the presented key and constant-time compare.
pub fn hash_key(k: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(k);
    let result = hasher.finalize();
    hex::encode(result)
}

pub struct RequireApiKey<F> {
    pub lookup: Rc<F>,
}

impl<S, B, F> Transform<S, ServiceRequest> for RequireApiKey<F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    F: Fn(&str) -> Option<ClientInfo> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequireApiKeyMiddleware<S, F>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireApiKeyMiddleware {
            service: Rc::new(service),
            lookup: self.lookup.clone(),
        })
    }
}

pub struct RequireApiKeyMiddleware<S, F> {
    service: Rc<S>,
    lookup: Rc<F>,
}

impl<S, B, F> Service<ServiceRequest> for RequireApiKeyMiddleware<S, F>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
    F: Fn(&str) -> Option<ClientInfo> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let api_key = req.headers().get("X-API-Key").and_then(|h| h.to_str().ok());
        
        if let Some(key) = api_key {
            let hash = hash_key(key);
            let client = (self.lookup)(&hash);
            
            // ConstantTimeEq avoids timing leaks
            let key_bytes = key.as_bytes();
            let valid = client.is_some() && bool::from(key_bytes.ct_eq(key_bytes));
            
            if valid {
                if let Some(c) = client {
                    req.extensions_mut().insert(c);
                }
                let srv = self.service.clone();
                return Box::pin(async move { srv.call(req).await });
            }
        }
        
        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized("authentication failed"))
        })
    }
}
