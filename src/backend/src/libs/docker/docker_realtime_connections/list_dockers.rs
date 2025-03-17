use std::collections::HashMap;

use crate::{error::Error, libs::docker::{get_docker_connections_db::get_docker_connections_db, structs::docker_realtime_connection::DockerRealtimeConnection}};
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn list_dockers(&self) -> Result<HashMap<String, DockerRealtimeConnection>, Error> {
        let guard = self.inner.read().await;
        let connection_error_guard = self.connection_errors.read().await;
        
        let mut active_dockers = HashMap::<String, DockerRealtimeConnection>::new();
        let all_dockers = get_docker_connections_db().await?;
        
        for docker_connection in all_dockers {
            let connection_error = connection_error_guard.get(&docker_connection.name).cloned();
            active_dockers.insert(docker_connection.name.clone(), DockerRealtimeConnection::from((docker_connection, false, connection_error)));
        }

        for (name, (docker_connection, _)) in guard.iter() {
            active_dockers.insert(name.clone(), DockerRealtimeConnection::from((docker_connection.clone(), true, None)));
        }

        Ok(active_dockers)
    }    
}