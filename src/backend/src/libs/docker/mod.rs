use std::{collections::HashMap, sync::Arc};

use bollard::Docker;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

lazy_static! {
    pub static ref REALTIME_CONNECTED_DOCKERS: Arc<RwLock<HashMap<String, Docker>>> = Arc::new(RwLock::new(HashMap::<String, Docker>::new()));
}

pub mod get_docker_connections_db;
pub mod check_local_connection_exist_db;
pub mod store_docker_connection;
pub mod is_docker_active;
pub mod list_containers;
pub mod connect_to_docker;
pub mod add_new_docker;
pub mod structs;
pub mod init_dockers;
