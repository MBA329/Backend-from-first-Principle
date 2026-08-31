// ===== SERVER =====
use tonic::{transport::Server, Request, Response, Status, Code};
use userv1::user_service_server::{UserService, UserServiceServer};
use userv1::{GetUserRequest, GetUserResponse, User, Role};

// generated from user.proto
pub mod userv1 {
    tonic::include_proto!("user.v1");
}

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl UserService for MyServer {
    // The method signature is generated FROM the proto: req -> Response, Status
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        
        if req.id.is_empty() {
            // typed error, sec 14
            return Err(Status::new(Code::InvalidArgument, "id is required"));
        }
        
        // ...real work: query the DB by req.id...
        let u = User {
            id: req.id,
            email: "ada@example.com".to_string(),
            full_name: "Ada".to_string(),
            role: Role::Admin as i32,
            tags: vec![],
            created_at: 0,
        };
        
        Ok(Response::new(GetUserResponse { user: Some(u) }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let server = MyServer::default();

    println!("gRPC on :50051");
    // wire the impl to the service
    Server::builder()
        .add_service(UserServiceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

// ===== CLIENT =====
use userv1::user_service_client::UserServiceClient;
use std::time::Duration;

#[tokio::main]
async fn call_get_user() -> Result<(), Box<dyn std::error::Error>> {
    // insecure creds for local dev only (Tonic defaults to insecure unless TLS is configured)
    let mut client = UserServiceClient::connect("http://[::1]:50051").await?;
    
    let mut req = Request::new(GetUserRequest {
        id: "u42".to_string(),
    });
    // always a deadline, sec 13
    req.set_timeout(Duration::from_secs(1));

    // looks local, runs remote
    match client.get_user(req).await {
        Ok(resp) => {
            let u = resp.into_inner().user.unwrap();
            println!("got user: {}", u.full_name);
        }
        Err(err) => {
            // err carries the gRPC status code
            eprintln!("GetUser failed: {}", err);
        }
    }
    
    Ok(())
}
