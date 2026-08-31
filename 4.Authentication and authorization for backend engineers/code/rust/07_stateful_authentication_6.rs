use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub role: String,
}

pub struct RequireSession {
    pub rdb: Client,
}

impl<S, B> Transform<S, ServiceRequest> for RequireSession
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequireSessionMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequireSessionMiddleware { 
            service: Rc::new(service),
            rdb: self.rdb.clone()
        })
    }
}

pub struct RequireSessionMiddleware<S> {
    service: Rc<S>,
    rdb: Client,
}

impl<S, B> Service<ServiceRequest> for RequireSessionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // RequireSession reads the sid cookie and looks it up in Redis.
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let cookie = req.cookie("sid");
        if cookie.is_none() {
            return Box::pin(async move {
                Err(actix_web::error::ErrorUnauthorized("authentication failed"))
            });
        }
        
        let sid = cookie.unwrap().value().to_string();
        let srv = self.service.clone();
        let rdb = self.rdb.clone();

        Box::pin(async move {
            let mut con = rdb.get_async_connection().await.map_err(actix_web::error::ErrorInternalServerError)?;
            let raw: Option<String> = con.get(format!("sess:{}", sid)).await.map_err(actix_web::error::ErrorInternalServerError)?;
            
            if let Some(r) = raw {
                if let Ok(u) = serde_json::from_str::<User>(&r) {
                    // attach the user to the request context for later handlers
                    req.extensions_mut().insert(u);
                    return srv.call(req).await;
                }
            }
            // missing or expired
            Err(actix_web::error::ErrorUnauthorized("authentication failed"))
        })
    }
}
