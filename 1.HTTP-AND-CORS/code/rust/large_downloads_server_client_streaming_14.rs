use actix_web::{web, HttpResponse, Responder};
use futures_util::stream::StreamExt as _;
use std::time::Duration;

// 1. Receive a multipart upload (stubbed for conceptual translation)
async fn upload(mut payload: web::Payload) -> impl Responder {
    // Actix payload handles streaming data for multipart
    let mut bytes = web::BytesMut::new();
    while let Some(item) = payload.next().await {
        bytes.extend_from_slice(&item.unwrap());
    }
    HttpResponse::Ok().body("received file")
}

// 2. Stream a response in chunks (Server-Sent Events)
async fn stream() -> impl Responder {
    let stream = async_stream::stream! {
        for i in 0..5 {
            yield Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: chunk {}\n\n", i)));
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    };
    
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .insert_header(("Connection", "keep-alive"))
        .streaming(stream)
}
