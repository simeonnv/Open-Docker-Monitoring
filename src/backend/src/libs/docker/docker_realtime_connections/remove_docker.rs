use crate::{error::Error, libs::docker::remove_docker_connection_db::remove_docker_connection_db};
use super::DockerRealtimeConnections;


impl DockerRealtimeConnections {
    pub async fn remove_docker(&self, name: String) -> Result<(), Error> {
        
        remove_docker_connection_db(&name).await?;

        let mut error_guard = self.connection_errors.write().await;
        let mut docker_connection_guard = self.inner.write().await;

        error_guard.remove(&name);
        docker_connection_guard.remove(&name);

        Ok(())
    }    
}