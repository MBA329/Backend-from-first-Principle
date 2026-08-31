use futures::{SinkExt, StreamExt};
use http::Request;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn run_client() {
    let token = "your-token";
    
    // Dial performs the upgrade handshake; header carries auth where allowed (sec 13).
    let request = Request::builder()
        .uri("wss://api.example.com/ws")
        .header("Authorization", format!("Bearer {}", token))
        .body(())
        .unwrap();

    let (ws_stream, _) = connect_async(request).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    // reader task
    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            if let Ok(msg) = message {
                println!("recv: {:?}", msg);
            } else {
                return;
            }
        }
    });

    // send a message
    write.send(Message::Text(r#"{"type":"hello"}"#.to_string())).await.unwrap();

    // graceful close: send a close frame, then the socket shuts (sec 6)
    write.send(Message::Close(Some(tokio_tungstenite::tungstenite::protocol::CloseFrame {
        code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Normal,
        reason: std::borrow::Cow::Borrowed("bye"),
    }))).await.unwrap();
}
