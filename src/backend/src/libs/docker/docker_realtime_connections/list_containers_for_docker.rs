use bollard::container::ListContainersOptions;
use crate::{error::Error, libs::docker::structs::container_refined_summary::ContainerRefinedSummary};
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn list_containers_for_docker(&self, name: &String) -> Result<Vec<ContainerRefinedSummary>, Error> {
        let guard = self.inner.read().await;
        
        let (_, docker) = match guard.get(name) {
            Some(e) => e,
            None => return Err(Error::BadRequest("Docker does not exist with such name!".to_string())),
        };

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let containers = docker.list_containers(Some(options)).await?
            .into_iter()
            .map(|i| {
                i.into()
            })
            .collect();

        Ok(containers)
    }    
}