use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use jsonwebtoken::{decode, Validation, DecodingKey};
use serde::{Deserialize, Serialize};

const SECRET: &[u8] = b"keep-this-very-secret";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub role: String,
}

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub role: String,
}

// verify signature + expiry; returns the claims if valid
fn verify(token_str: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.validate_exp = true;
    
    let token_data = decode::<Claims>(
        token_str,
        &DecodingKey::from_secret(SECRET),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

pub struct RequireJwt;

impl<S, B> Transform<S, ServiceRequest> for RequireJwt
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequireJwtMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireJwtMiddleware { service: Rc::new(service) })
    }
}

pub struct RequireJwtMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RequireJwtMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(header) = auth_header {
            if let Ok(header_str) = header.to_str() {
                if header_str.starts_with("Bearer ") {
                    let token = &header_str[7..];
                    if let Ok(claims) = verify(token) {
                        // no store lookup ,  identity comes from the verified token
                        let u = User {
                            id: claims.sub.clone(),
                            role: claims.role.clone(),
                        };
                        req.extensions_mut().insert(u);
                        
                        let srv = self.service.clone();
                        return Box::pin(async move { srv.call(req).await });
                    }
                }
            }
        }
        
        Box::pin(async move {
            Err(actix_web::error::ErrorUnauthorized("authentication failed"))
        })
    }
}
