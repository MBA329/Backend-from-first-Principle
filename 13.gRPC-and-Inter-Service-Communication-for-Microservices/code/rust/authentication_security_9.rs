use tonic::transport::{Certificate, Identity, Server, ClientTlsConfig, Channel};
use tonic::metadata::MetadataValue;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ===== SERVER over TLS =====
    /* 
    let cert = fs::read("server.crt")?;
    let key = fs::read("server.key")?;
    let identity = Identity::from_pem(cert, key);
    
    let tls_config = tonic::transport::ServerTlsConfig::new()
        .identity(identity);

    // every connection is now encrypted
    Server::builder()
        .tls_config(tls_config)?
        .add_service(svc)
        .serve(addr)
        .await?;
    */

    // ===== CLIENT over TLS =====
    /*
    let root_ca = fs::read("ca.crt")?;
    let cert = Certificate::from_pem(root_ca); // trust this CA
    
    let tls_config = ClientTlsConfig::new()
        .ca_certificate(cert)
        .domain_name("api.example.com");

    let channel = Channel::from_static("https://api.example.com:443")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut client = UserServiceClient::new(channel);
    */

    // ===== Per-call token (combine with TLS) =====
    // Use an interceptor to attach the token so a fresh token rides on every call:
    let token = "my-secret-token";
    let token_val: MetadataValue<_> = format!("Bearer {}", token).parse()?;
    
    let auth_interceptor = move |mut req: tonic::Request<()>| {
        req.metadata_mut().insert("authorization", token_val.clone());
        Ok(req)
    };

    // let mut client_with_token = UserServiceClient::with_interceptor(channel, auth_interceptor);

    Ok(())
}
