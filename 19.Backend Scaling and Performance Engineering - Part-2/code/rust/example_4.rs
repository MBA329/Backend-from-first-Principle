use actix_web::{get, HttpResponse, Responder};
use prometheus::{HistogramVec, HistogramOpts, register_histogram_vec};
use lazy_static::lazy_static;

lazy_static! {
    static ref REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        HistogramOpts::new("http_request_duration_seconds", "Duration of HTTP requests"),
        &["method", "path", "status"]
    ).unwrap();
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    use prometheus::Encoder;
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = vec![];
    encoder.encode(&prometheus::gather(), &mut buffer).unwrap();
    HttpResponse::Ok().body(buffer)
}
