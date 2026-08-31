use tonic::{Request, Status, service::Interceptor};

// tonic interceptors run before the request is routed to the handler

// A unary server interceptor: signature is fixed by the framework.
fn auth_interceptor(mut req: Request<()>) -> Result<Request<()>, Status> {
    match req.metadata().get("authorization") {
        Some(t) if valid(t) => {
            // attach the verified identity for the handler to read from ctx
            let user = parse(t);
            req.extensions_mut().insert(user);
            Ok(req) // call the next link / the real handler
        }
        _ => Err(Status::unauthenticated("missing or invalid token")),
    }
}

// For logging in Tonic, you usually use a `tower` middleware layer instead 
// of an interceptor, because interceptors can't easily run code *after* the handler.
// Here's the tower middleware concept:
//
// let layer = tower::ServiceBuilder::new()
//     .layer(tower_http::trace::TraceLayer::new_for_grpc())
//     .into_inner();

// Register the chain when building the server (outermost listed first):
// let svc = UserServiceServer::with_interceptor(MyServer::default(), auth_interceptor);
// Server::builder()
//     .layer(layer)
//     .add_service(svc)
//     .serve(addr)
//     .await?;

// Dummies
fn valid(t: &tonic::metadata::MetadataValue<tonic::metadata::Ascii>) -> bool { true }
#[derive(Clone, Copy, Send, Sync, 'static)]
pub struct UserKey;
fn parse(t: &tonic::metadata::MetadataValue<tonic::metadata::Ascii>) -> UserKey { UserKey }
