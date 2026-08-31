use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use std::rc::Rc;
use std::collections::HashSet;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub role: String,
}

pub struct RequireRole {
    pub roles: Vec<String>,
}

impl<S, B> Transform<S, ServiceRequest> for RequireRole
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequireRoleMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let mut allowed = HashSet::new();
        for r in &self.roles {
            allowed.insert(r.clone());
        }
        
        ok(RequireRoleMiddleware {
            service: Rc::new(service),
            allowed: Rc::new(allowed),
        })
    }
}

pub struct RequireRoleMiddleware<S> {
    service: Rc<S>,
    allowed: Rc<HashSet<String>>,
}

impl<S, B> Service<ServiceRequest> for RequireRoleMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    // RequireRole runs AFTER an auth middleware that set "user".
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let user = req.extensions().get::<User>().cloned();
        
        if let Some(u) = user {
            if self.allowed.contains(&u.role) {
                let srv = self.service.clone();
                return Box::pin(async move { srv.call(req).await });
            }
        }
        
        Box::pin(async move {
            Err(actix_web::error::ErrorForbidden("forbidden")) // 403
        })
    }
}
