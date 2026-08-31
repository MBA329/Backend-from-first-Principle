use tokio::net::TcpListener;

// Tokio provides async runtime similar to Go's goroutines
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        // Create a NEW task (green thread) for each connection
        tokio::spawn(async move {
            handle_conn(socket).await;
        });
    }
}

async fn handle_conn(socket: tokio::net::TcpStream) {
    // Handle connection
}
