use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn is_docker_active(&self, name: &String) -> bool {
        let dockers = self.inner.read().await;
        dockers.contains_key(name)
    }    
}