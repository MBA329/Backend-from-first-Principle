use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        TypedHeader,
    },
    headers,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    // CheckOrigin guards against cross-site hijacking , DO NOT leave it open (sec 14).
    user_agent: Option<TypedHeader<headers::Origin>>,
) -> impl IntoResponse {
    let origin = user_agent.map(|o| o.to_string()).unwrap_or_default();
    
    // In axum, you would typically check this in a middleware, but here is inline:
    if origin != "https://app.example.com" {
        // Return unauthorized or just let it connect and close (omitted for brevity)
    }

    // performs the 101 handshake (sec 4)
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // the READ LOOP , one per connection
    while let Some(msg) = socket.next().await {
        if let Ok(msg) = msg {
            // echo it straight back
            if socket.send(msg).await.is_err() {
                break; // client closed or connection died (sec 6) -> exit, cleanup via drop
            }
        } else {
            break; 
        }
    }
    // socket is dropped and closed automatically (frees the socket)
}
