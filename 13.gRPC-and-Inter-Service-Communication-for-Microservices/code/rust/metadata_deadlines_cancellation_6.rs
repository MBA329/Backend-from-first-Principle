use tonic::{Request, Response, Status, metadata::MetadataValue};
use std::time::Duration;

// ===== CLIENT: attach a deadline + metadata =====
async fn call_with_meta(mut client: AnyServiceClient) {
    let mut req = Request::new(GetUserRequest { id: "u42".to_string() });
    
    // absolute deadline
    req.set_timeout(Duration::from_secs(1));
    
    let token = "my-token";
    let req_id = "12345";
    
    // auth travels as metadata, sec 16
    let auth_val: MetadataValue<_> = format!("Bearer {}", token).parse().unwrap();
    req.metadata_mut().insert("authorization", auth_val);
    
    // trace id for correlation, like sec 15 of the layers manual
    req.metadata_mut().insert("x-request-id", req_id.parse().unwrap());
    
    match client.get_user(req).await {
        Err(e) if e.code() == tonic::Code::DeadlineExceeded => {
            println!("call timed out"); // a network-shaped failure a local call never had
        }
        _ => {}
    }
}

// ===== SERVER: read metadata, respect the inherited deadline =====
#[tonic::async_trait]
impl MyServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        let md = request.metadata();
        let auth = md.get("authorization"); // verify token here (or in an interceptor, sec 15)

        // In Tonic/Tokio, cancellation is handled by dropping the future.
        // If the client disconnects or times out, the `get_user` future is dropped.
        // pass it straight to the DB driver so slow work is abandoned automatically.

        let _ = auth;
        let id = request.into_inner().id;
        self.lookup(&id).await
    }
}

// Dummy structs
pub struct GetUserRequest { pub id: String }
pub struct GetUserResponse {}
pub struct AnyServiceClient;
impl AnyServiceClient { async fn get_user(&mut self, req: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> { unimplemented!() } }
pub struct MyServer;
impl MyServer { async fn lookup(&self, id: &str) -> Result<Response<GetUserResponse>, Status> { unimplemented!() } }
