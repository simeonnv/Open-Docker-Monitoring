use super::DockerRealtimeConnections;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;


impl DockerRealtimeConnections {
    pub fn new() -> Self {
        DockerRealtimeConnections {
            inner: Arc::new(RwLock::new(HashMap::new())),
            connection_errors: Arc::new(RwLock::new(HashMap::new()))
        }
    }    
}