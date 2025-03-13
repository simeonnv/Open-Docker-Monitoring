use bollard::{container::ListContainersOptions, secret::ContainerSummary};
use crate::error::Error;
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn list_containers_for_docker(&self, name: &String) -> Result<Vec<ContainerSummary>, Error> {
        let guard = self.inner.read().await;
        
        let (_, docker) = match guard.get(name) {
            Some(e) => e,
            None => return Err(Error::BadRequest("Docker does not exist with such name!".to_string())),
        };

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        Ok(docker.list_containers(Some(options)).await?)
    }    
}