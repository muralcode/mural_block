//Copyright (c) 2024 Arithoptix Pty Ltd.
use std::collections::HashMap;
use log::{info, error};
use std::sync::{Arc, RwLock};
use tokio::task;

pub struct StorageManager {
    store: Arc<RwLock<HashMap<String, String>>>,
    node_resources: Arc<RwLock<HashMap<usize, ResourceAllocation>>>,
}

#[derive(Debug, Clone)]
pub struct ResourceAllocation {
    pub cpu: usize,
    pub memory: usize,
    pub storage: usize,
}

impl StorageManager {
    pub fn new() -> Self {
        StorageManager {
            store: Arc::new(RwLock::new(HashMap::new())),
            node_resources: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn save(&self, key: String, value: String) {
        let store = self.store.clone();
        task::spawn_blocking(move || {
            let mut store = store.write().unwrap();
            store.insert(key, value);
            info!("Data saved successfully.");
        }).await.unwrap_or_else(|e| error!("Failed to save data: {}", e));
    }

    pub async fn load(&self, key: &str) -> Option<String> {
        let store = self.store.clone();
        let key = key.to_string();
        task::spawn_blocking(move || {
            let store = store.read().unwrap();
            store.get(&key).cloned()
        }).await.unwrap_or_else(|e| {
            error!("Failed to load data: {}", e);
            None
        })
    }

    pub async fn allocate_resources(&self, node_id: usize, cluster_id: usize, allocation: ResourceAllocation) {
        let node_resources = self.node_resources.clone();
        task::spawn_blocking(move || {
            let mut resources = node_resources.write().unwrap();
            resources.insert(node_id, allocation);
            info!("Resources allocated for Node {} in Cluster {}", node_id, cluster_id);
        }).await.unwrap_or_else(|e| error!("Failed to allocate resources: {}", e));
    }

    pub async fn get_resource_allocation(&self, node_id: usize) -> Option<ResourceAllocation> {
        let node_resources = self.node_resources.clone();
        task::spawn_blocking(move || {
            let resources = node_resources.read().unwrap();
            resources.get(&node_id).cloned()
        }).await.unwrap_or_else(|e| {
            error!("Failed to get resource allocation: {}", e);
            None
        })
    }

    pub async fn update_resource_allocation(&self, node_id: usize, allocation: ResourceAllocation) {
        let node_resources = self.node_resources.clone();
        task::spawn_blocking(move || {
            let mut resources = node_resources.write().unwrap();
            if let Some(existing_allocation) = resources.get_mut(&node_id) {
                *existing_allocation = allocation;
                info!("Resources updated for Node {}", node_id);
            } else {
                error!("Node {} not found for resource update", node_id);
            }
        }).await.unwrap_or_else(|e| error!("Failed to update resource allocation: {}", e));
    }
}
