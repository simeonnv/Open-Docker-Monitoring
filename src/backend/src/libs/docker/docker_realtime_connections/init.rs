use bollard::Docker;

use crate::{error::Error, libs::docker::{check_docker_hearthbeat::check_docker_hearthbeat, connect_to_docker::connect_to_docker, get_docker_connections_db::get_docker_connections_db}};
use super::DockerRealtimeConnections;


impl DockerRealtimeConnections {
    pub async fn init(&self) -> Result<(), Error> {
        
        let docker_connections = get_docker_connections_db().await?;

        for docker_connection in docker_connections {
            
            let docker: Docker = connect_to_docker(&docker_connection).await?;
            
            match check_docker_hearthbeat(&docker).await {
                Ok(e) => e,
                Err(e) => {
                    self.connection_errors.write().await.insert(docker_connection.name.clone(), e.to_string());
                    continue
                }
            };
            
            self.insert(docker_connection.name.clone(),  docker_connection, docker).await;
        } 
        // // ! this could lead to error in the close future
        // // ! if a docker stored from the database failes to load on runtime, it will lead to a whole exeption crash
        // // ! if not fixed can softlock the entire server
    
        Ok(())
    }    
}