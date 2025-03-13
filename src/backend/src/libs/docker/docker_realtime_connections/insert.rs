use crate::libs::docker::structs::docker_connection::DockerConnection;
use super::DockerRealtimeConnections;
use bollard::Docker;


impl DockerRealtimeConnections {
    pub async fn insert(&self, name: String, docker_connection_info: DockerConnection, docker: Docker) -> () {
        self.inner.write().await.insert(name, (docker_connection_info, docker));
    }    
}