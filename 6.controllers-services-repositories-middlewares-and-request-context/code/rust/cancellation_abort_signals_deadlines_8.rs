use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage};
use actix_web::dev::Service;
use futures_util::future::LocalBoxFuture;
use uuid::Uuid;

pub struct RequestIdMiddleware<S> { service: S }

impl<S, B> Service<ServiceRequest> for RequestIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let id = Uuid::new_v4().to_string(); // one unique id for this request
        req.extensions_mut().insert(id.clone());
        println!("[{}] {} {}", id, req.method(), req.path());
        
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            res.headers_mut().insert(
                actix_web::http::header::HeaderName::from_static("x-request-id"),
                actix_web::http::header::HeaderValue::from_str(&id).unwrap(),
            );
            Ok(res)
        })
    }
}
