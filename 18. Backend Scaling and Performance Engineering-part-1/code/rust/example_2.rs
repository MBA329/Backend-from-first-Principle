use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use std::{future::{ready, Ready}, time::Instant};
use futures_util::future::LocalBoxFuture;

pub struct TimingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for TimingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TimingMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future { ready(Ok(TimingMiddlewareService { service })) }
}

pub struct TimingMiddlewareService<S> { service: S }

impl<S, B> Service<ServiceRequest> for TimingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = Instant::now();
        let method = req.method().clone();
        let path = req.path().to_owned();
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            let duration = start.elapsed();
            println!("method={} path={} status={} duration={:?}", method, path, res.status(), duration);
            Ok(res)
        })
    }
}
