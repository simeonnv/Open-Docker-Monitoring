use std::{collections::HashMap, sync::Arc};

use bollard::Docker;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    pub static ref REALTIME_CONNECTED_DOCKERS: Arc<Mutex<HashMap<String, Docker>>> = Arc::new(Mutex::new(HashMap::<String, Docker>::new()));
}


pub mod get_docker_connections_db;
pub mod check_local_connection_exist;
pub mod store_docker_connection;
pub mod create_docker_listiner_worker;
pub mod is_docker_active;
pub mod list_containers;
pub mod spawn_log_container_task;