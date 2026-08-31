// proto:  rpc UploadEvents(stream Event) returns (UploadSummary);

use tonic::{Request, Response, Status};

// Dummy types
pub struct Event {}
pub struct UploadSummary { pub received: i32 }
pub struct MyServer;
fn store(e: Event) {}

// ===== SERVER: next() in a loop, then Ok(Response) once at the end =====
#[tonic::async_trait]
impl MyServer {
    async fn upload_events(
        &self,
        request: Request<tonic::Streaming<Event>>,
    ) -> Result<Response<UploadSummary>, Status> {
        let mut stream = request.into_inner();
        let mut count = 0;
        
        while let Some(ev) = stream.message().await? {
            // client finished sending when stream.message() returns None, now reply once
            store(ev);
            count += 1;
        }
        
        Ok(Response::new(UploadSummary { received: count }))
    }
}

// ===== CLIENT: stream of requests, await the single reply =====
async fn upload_events(mut client: AnyServiceClient, events: Vec<Event>) {
    // create a stream of requests
    let iter = events.into_iter().map(Ok);
    let request_stream = futures::stream::iter(iter);
    
    let mut req = Request::new(request_stream);
    req.set_timeout(std::time::Duration::from_secs(30));

    // close our side, await the summary
    let summary = client.upload_events(req).await.unwrap().into_inner();
    
    println!("server received {} events", summary.received);
}
struct AnyServiceClient;
impl AnyServiceClient { async fn upload_events<T>(&mut self, req: Request<T>) -> Result<Response<UploadSummary>, Status> { unimplemented!() } }
