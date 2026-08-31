use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_web::dev::Transform;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::dev::Service;

const ALLOWED_ORIGIN: &str = "https://app.example.com";

// (CORS is typically handled by actix_cors::Cors, but here is a middleware conceptual sketch)

// Auth Middleware concept
pub struct AuthMiddleware<S> { service: S }

#[derive(Clone)]
pub struct AuthCtx {
    pub user_id: i32,
    pub role: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let token = req.headers().get("Authorization");
        
        // Sketch verify_token
        let is_valid = true; 
        
        if !is_valid {
            // return 401
        }
        
        // SUCCESS: stash identity in the request context, then continue.
        req.extensions_mut().insert(AuthCtx { user_id: 1, role: "user".to_string() });

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
