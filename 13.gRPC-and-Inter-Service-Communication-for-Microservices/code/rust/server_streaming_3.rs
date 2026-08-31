// proto:  rpc ListUsers(ListUsersRequest) returns (stream User);

use tonic::{Request, Response, Status};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

// Dummy types
pub struct ListUsersRequest { pub filter: String }
pub struct User { pub full_name: String }
pub struct MyServer;
fn query_users(f: String) -> Vec<User> { vec![] }
pub trait UserService {}

// ===== SERVER: receive one req, call stream.Send(...) repeatedly =====
#[tonic::async_trait]
impl MyServer {
    // specify the stream type returned
    type ListUsersStream = ReceiverStream<Result<User, Status>>;

    async fn list_users(
        &self,
        request: Request<ListUsersRequest>,
    ) -> Result<Response<Self::ListUsersStream>, Status> {
        let filter = request.into_inner().filter;
        
        // imagine this yields a big result set
        let users = query_users(filter);
        
        // create a channel to stream results
        let (tx, rx) = mpsc::channel(4);
        
        tokio::spawn(async move {
            for u in users {
                // push one message down the stream
                if tx.send(Ok(u)).await.is_err() {
                    break; // client gone / cancelled
                }
            }
            // returning nil (dropping tx) closes the stream cleanly (sends OK trailer)
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

// ===== CLIENT: call once, then stream.message() in a loop until None =====
async fn list_users(mut client: AnyServiceClient) {
    let mut req = Request::new(ListUsersRequest {
        filter: "active".to_string(),
    });
    req.set_timeout(std::time::Duration::from_secs(10));

    let mut stream = client.list_users(req).await.unwrap().into_inner();
    
    while let Some(u) = stream.message().await.unwrap() {
        // server closed the stream ,  we're done when stream.message() returns None
        println!("user: {}", u.full_name);
    }
}
struct AnyServiceClient;
impl AnyServiceClient { async fn list_users(&mut self, req: Request<ListUsersRequest>) -> Result<Response<tonic::Streaming<User>>, Status> { unimplemented!() } }
