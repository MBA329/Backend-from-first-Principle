// proto:  rpc Chat(stream ChatMessage) returns (stream ChatMessage);

use tonic::{Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct ChatMessage { pub user: String, pub text: String }
pub struct MyServer;

// ===== SERVER: loop next() and send on the same stream =====
#[tonic::async_trait]
impl MyServer {
    type ChatStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn chat(
        &self,
        request: Request<tonic::Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut stream = request.into_inner();
        let (tx, rx) = mpsc::channel(10);
        
        tokio::spawn(async move {
            while let Ok(Some(msg)) = stream.message().await {
                // echo back (or broadcast to a room, etc.), can Send anytime, any number
                let reply = ChatMessage {
                    user: "server".to_string(),
                    text: format!("ack: {}", msg.text),
                };
                if tx.send(Ok(reply)).await.is_err() {
                    break;
                }
            }
            // client closed its send side
        });
        
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// ===== CLIENT: typically send in one task, recv in another =====
async fn chat(mut client: AnyServiceClient) {
    let (tx, rx) = mpsc::channel(10);
    
    // concurrent receiver via returned stream
    let mut stream = client.chat(Request::new(ReceiverStream::new(rx))).await.unwrap().into_inner();
    
    tokio::spawn(async move {
        while let Some(in_msg) = stream.message().await.unwrap() {
            println!("<< {}", in_msg.text);
        }
    });
    
    for t in &["hi", "how are you", "bye"] {
        let msg = ChatMessage {
            user: "ada".to_string(),
            text: t.to_string(),
        };
        tx.send(msg).await.unwrap(); // send concurrently
    }
    
    // signal we're done sending; receiver drains the rest by dropping tx
    drop(tx);
}
struct AnyServiceClient;
impl AnyServiceClient { async fn chat<T>(&mut self, req: Request<T>) -> Result<Response<tonic::Streaming<ChatMessage>>, Status> { unimplemented!() } }
