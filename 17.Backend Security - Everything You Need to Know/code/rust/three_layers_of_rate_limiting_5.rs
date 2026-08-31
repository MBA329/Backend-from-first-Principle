use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use once_cell::sync::Lazy;
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpResponse};
use std::future::{ready, Ready};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;

// Per-IP limiter: 5 requests per second, burst of 10
struct Limiter {
    tokens: f64,
    last_update: Instant,
}

impl Limiter {
    fn new() -> Self {
        Self { tokens: 10.0, last_update: Instant::now() }
    }
    fn allow(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs_f64();
        self.tokens = (self.tokens + elapsed * 5.0).min(10.0);
        self.last_update = now;
        
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

static LIMITERS: Lazy<Mutex<HashMap<String, Limiter>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn allow_ip(ip: &str) -> bool {
    let mut limiters = LIMITERS.lock().unwrap();
    let limiter = limiters.entry(ip.to_string()).or_insert_with(Limiter::new);
    limiter.allow()
}

// Actix middleware placeholder
pub struct RateLimitMiddleware;

impl<S, B> Transform<S, ServiceRequest> for RateLimitMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RateLimitMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RateLimitMiddlewareService { service: Rc::new(service) }))
    }
}

pub struct RateLimitMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for RateLimitMiddlewareService<S>
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
        let ip = req.peer_addr().map(|a| a.ip().to_string()).unwrap_or_else(|| "unknown".to_string());
        
        if !allow_ip(&ip) {
            let (request, _pl) = req.into_parts();
            let response = HttpResponse::TooManyRequests().body("too many requests").into_body();
            return Box::pin(ready(Ok(ServiceResponse::new(request, response))));
        }

        let srv = self.service.clone();
        Box::pin(async move {
            srv.call(req).await
        })
    }
}
