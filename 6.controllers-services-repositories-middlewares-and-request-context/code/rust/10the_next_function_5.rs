use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_web::dev::Transform;
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use actix_web::dev::Service;

// In Rust (Actix-web), middleware wraps the next service. Calling Service::call
// is the equivalent of next(): pass execution along the chain.
pub struct LoggingMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("{} {}", req.method(), req.path()); // do work

        // EARLY EXIT example (short-circuit, never calls next):
        if req.headers().get("X-Blocked").map(|h| h.to_str().unwrap_or("")) == Some("yes") {
            let (http_req, _payload) = req.into_parts();
            let res = HttpResponse::Forbidden().body("forbidden");
            // Must return matched generic B type
            // (skipped exact response mapping for sketch simplicity)
        }

        // next(): continue the chain
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
