//Copyright (c) 2024 Arithoptix Pty Ltd.
use tokio::sync::mpsc;
use serde::{Serialize, Deserialize};
use log::info;

#[derive(Serialize, Deserialize, Debug)]
struct ReplicationRequest {
    data: Vec<u8>,
}

pub struct ReplicationManager;

impl ReplicationManager {
    pub fn new() -> Self {
        ReplicationManager
    }

    pub async fn replicate(&self, data: Vec<u8>) {
        // basic usage: send data to peers
        let (tx, mut rx) = mpsc::channel(100);
        tx.send(ReplicationRequest { data }).await.expect("Failed to send replication request");

        while let Some(request) = rx.recv().await {
            info!("Replicating: {:?}", request);
        }
    }
}
