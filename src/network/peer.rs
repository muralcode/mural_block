use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use log::info;

pub struct Peer;

impl Peer {
    pub fn new() -> Self {
        Peer
    }

    pub async fn start(&self) {
        let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind to address");
        info!("Peer listening on 127.0.0.1:8080");

        loop {
            let (mut socket, _) = listener.accept().await.expect("Failed to accept connection");
            tokio::spawn(async move {
                let mut buf = [0; 1024];
                socket.read(&mut buf).await.expect("Failed to read from socket");
                socket.write_all(&buf).await.expect("Failed to write to socket");
            });
        }
    }
}
