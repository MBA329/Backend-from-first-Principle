use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, App, Error, HttpResponse, HttpServer};
use actix_web::http::header;
use futures_util::future::{ok, Ready};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::task::{Context, Poll};

// Actix-web provides a built-in middleware for security headers via DefaultHeaders, 
// but here is a custom middleware to match the concept:

pub struct SecurityHeaders;

impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SecurityHeadersMiddleware { service: Rc::new(service) })
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for SecurityHeadersMiddleware<S>
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
        let svc = self.service.clone();
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let headers = res.headers_mut();
            headers.insert(header::STRICT_TRANSPORT_SECURITY, "max-age=31536000; includeSubDomains".parse().unwrap());
            headers.insert(header::CONTENT_SECURITY_POLICY, "default-src 'self'".parse().unwrap());
            headers.insert(header::HeaderName::from_static("x-frame-options"), "DENY".parse().unwrap());
            headers.insert(header::X_CONTENT_TYPE_OPTIONS, "nosniff".parse().unwrap());
            Ok(res)
        })
    }
}
// App::new().wrap(SecurityHeaders)
