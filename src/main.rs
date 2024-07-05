//Copyright (c) 2024 Arithoptix Pty Ltd.
mod network;
mod blockchain;
mod storage;
mod adaptive_kmeans;

use adaptive_kmeans::{run_adaptive_kmeans, Point, KMeansResult};
use log::{error, info};
use network::peer::Peer;
use storage::{StorageManager, ResourceAllocation};
use std::{result, sync::Arc};

#[tokio::main]
async fn main() {
    // Initialize logger
    if let Err(e) = env_logger::try_init() {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }

    // Initialize components
    //let blockchain = Arc::new(Blockchain::new());
    let storage_manager =  Arc::new(StorageManager::new());
    let peer = Arc::new(Peer::new());

    // Start P2P networking asynchronously
    let _peer_task = tokio::spawn(async move {
            peer.start().await;
    });

    //Sample mock data for adaptivekmeans clustering
    let data = vec![
         vec![1.0, 2.0],
         vec![1.5, 1.8],
         vec![5.0, 8.0],
         vec![8.0, 8.0],
         vec![1.0, 0.6],
         vec![9.0, 11.0],
    ];


    // Run custom adaptivekmeans clustering
    let k = 2;
    let max_iterations = 100;
    let use_mahalanobis = false;

    match run_adaptive_kmeans(data.clone(), k, max_iterations, use_mahalanobis) {
        result => {
            let assignments = unsafe { std::slice::from_raw_parts(result.assignments, data.len()) };
            for (node_id, cluster_id) in assignments.iter().enumerate() {
                info!("Node {} assigned to cluster {}", node_id, cluster_id);
                adjust_resource_allocation(node_id, (*cluster_id).try_into().unwrap(), Arc::clone(&storage_manager)).await;
                // usage of blockchain
            }
        }
        Error => {
            error!("Error running adaptivekmeans clustering");

        }
    }

    // sample  usage of StorageManager
    storage_manager.save("key1".to_string(), "value1".to_string()).await;
    if let Some(value) = storage_manager.load("key1").await {
        println!("Loaded value: {}", value);
    }

    // Storage manager usage resource allocation based on clustering
    storage_manager.allocate_resources(1, 1, ResourceAllocation { cpu: 2, memory: 2048, storage: 100 }).await;
}

async fn adjust_resource_allocation(node_id: usize, cluster_id: usize, storage_manager:  Arc<StorageManager>) {
    /*
    We Adjust node's resource allocation based on cluster assignment
    */
    info!("Adjusting resource allocation for Node {} in Cluster {}", node_id, cluster_id);
    let allocation = ResourceAllocation { cpu: 2, memory: 2048, storage: 100 };
    storage_manager.update_resource_allocation(node_id, allocation).await;
}
