use bollard::Docker;
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn get_by_name(&self, name: &String) -> Option<&Docker> {
        self.inner.read().await.get(name)
    }    
}