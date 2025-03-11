use crate::error::Error;
use super::DockerRealtimeConnections;
use bollard::Docker;


impl DockerRealtimeConnections {
    pub async fn insert(&self, name: String, docker: Docker) -> Result<(), Error> {
            
        self.inner.write().await.insert(name, docker);
        
        Ok(())
    }    
}