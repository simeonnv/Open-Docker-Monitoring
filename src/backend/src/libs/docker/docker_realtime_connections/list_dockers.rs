use std::collections::HashMap;

use bollard::{container::ListContainersOptions, secret::ContainerSummary, system::Version};
use crate::error::Error;
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn list_dockers(&self) -> Result<HashMap<String, Version>, Error> {
        let guard = self.inner.read().await;
        
        let mut all_docker_data = HashMap::<String, Version>::new();

        for (name, docker) in guard.iter() {
            let docker_data = match docker.version().await {
                Err(e) => return Err(Error::Internal(format!("There was a error reading all dockers: {}", e))),
                Ok(e) => e
            };
            all_docker_data.insert(name.clone(), docker_data);
        }

        Ok(all_docker_data)
    }    
}