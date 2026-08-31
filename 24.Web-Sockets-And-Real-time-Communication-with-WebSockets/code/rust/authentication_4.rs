use axum::{
    extract::{Query, WebSocketUpgrade, ws::WebSocket},
    response::{IntoResponse, Response},
    http::StatusCode,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthQuery {
    token: Option<String>,
}

struct User {
    id: i32,
    name: String,
}

fn verify_jwt(token: &str) -> Result<User, ()> {
    // your auth logic (auth chapter)
    if token == "valid" {
        Ok(User { id: 1, name: "User".to_string() })
    } else {
        Err(())
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    // AUTHENTICATE FIRST , before upgrading. Reject with a normal HTTP error.
    query: Query<AuthQuery>, // or read a cookie / subprotocol
) -> Response {
    let token = query.token.clone().unwrap_or_default();
    
    let user = match verify_jwt(&token) {
        Ok(u) => u,
        Err(_) => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(), // 401, no upgrade
    };

    // Only now perform the upgrade; attach the identity to the connection.
    ws.on_upgrade(move |socket| serve(socket, user))
}

async fn serve(mut socket: WebSocket, user: User) {
    // every message from this conn is now tied to `user`
    while let Some(Ok(msg)) = socket.recv().await {
        println!("Message from {}: {:?}", user.name, msg);
    }
}
