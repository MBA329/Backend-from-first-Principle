use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use actix_web::http::Method;
use futures_util::future::{ok, Ready, LocalBoxFuture};
use std::rc::Rc;
use std::task::{Context, Poll};

// Actix-web provides actix-cors, but here is a custom middleware to match the concept:
pub struct Cors;

impl<S, B> Transform<S, ServiceRequest> for Cors
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CorsMiddleware { service: Rc::new(service) })
    }
}

pub struct CorsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CorsMiddleware<S>
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
        let is_options = req.method() == Method::OPTIONS;

        if is_options {
            let (req, _pl) = req.into_parts();
            let mut res = HttpResponse::NoContent().finish(); // 204: answer the preflight and stop
            let headers = res.headers_mut();
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://example.com".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());
            
            return Box::pin(async { Ok(ServiceResponse::new(req, res.map_into_right_body())) });
        }

        let svc = self.service.clone();
        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let headers = res.headers_mut();
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://example.com".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH, DELETE".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS, "true".parse().unwrap());
            headers.insert(actix_web::http::header::ACCESS_CONTROL_MAX_AGE, "86400".parse().unwrap());
            Ok(res.map_into_left_body())
        })
    }
}
