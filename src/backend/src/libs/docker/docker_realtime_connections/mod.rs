use std::{collections::HashMap, sync::Arc};

use bollard::Docker;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

pub struct DockerRealtimeConnections {
    inner: Arc<RwLock<HashMap<String, Docker>>>, //this is private and not indended to be read (stiga s toq OOP)
}

pub mod new;
pub mod init;
pub mod insert;
pub mod new_docker;
pub mod is_docker_active;
pub mod list_containers_for_docker;
pub mod list_containers_all;

lazy_static! {
    pub static ref REALTIME_CONNECTED_DOCKERS: DockerRealtimeConnections = DockerRealtimeConnections::new();
}