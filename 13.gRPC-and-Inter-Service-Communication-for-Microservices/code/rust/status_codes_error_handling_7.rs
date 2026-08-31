use tonic::{Request, Response, Status, Code};

// SERVER: return a typed status, not a bare error string.
#[tonic::async_trait]
impl MyServer {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> {
        let req = request.into_inner();
        
        if req.id.is_empty() {
            return Err(Status::new(Code::InvalidArgument, "id is required"));
        }
        
        let u = self.db.find(&req.id);
        if u.is_none() {
            return Err(Status::new(Code::NotFound, format!("no user with id {:?}", req.id)));
        }
        
        Ok(Response::new(GetUserResponse { user: u }))
    }
}

// CLIENT: inspect the code to decide what to do.
async fn client_call(mut client: AnyServiceClient, req: Request<GetUserRequest>) {
    match client.get_user(req).await {
        Ok(resp) => { /* use resp */ }
        Err(st) => {
            // pull the status out of the error (st is already Status)
            match st.code() {
                Code::NotFound => {
                    // expected ,  show "not found" in UI
                }
                Code::Unavailable => {
                    // transient ,  retry with backoff, sec 17
                }
                _ => {
                    println!("unexpected: {:?}: {}", st.code(), st.message());
                }
            }
        }
    }
}

// Dummies
pub struct GetUserRequest { pub id: String }
pub struct GetUserResponse { pub user: Option<User> }
pub struct User {}
pub struct MyServer { db: Db }
pub struct Db {}
impl Db { fn find(&self, id: &str) -> Option<User> { None } }
pub struct AnyServiceClient;
impl AnyServiceClient { async fn get_user(&mut self, req: Request<GetUserRequest>) -> Result<Response<GetUserResponse>, Status> { unimplemented!() } }
