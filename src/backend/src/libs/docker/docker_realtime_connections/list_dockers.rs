use std::collections::HashMap;

use crate::{error::Error, libs::docker::structs::{docker_connection::DockerConnection, docker_info::DockerInfo}};
use super::DockerRealtimeConnections;

impl DockerRealtimeConnections {
    pub async fn list_dockers(&self) -> Result<HashMap<String, (DockerConnection, DockerInfo)>, Error> {
        let guard = self.inner.read().await;
        
        let mut all_docker_data = HashMap::<String, (DockerConnection, DockerInfo)>::new();

        for (name, (docker_connection_info, docker)) in guard.iter() {
            let docker_data = match docker.info().await {
                Err(e) => return Err(Error::Internal(format!("There was a error reading all dockers: {}", e))),
                Ok(e) => e
            };
            all_docker_data.insert(name.clone(), (docker_connection_info.clone(), docker_data.into()));
        }

        Ok(all_docker_data)
    }    
}