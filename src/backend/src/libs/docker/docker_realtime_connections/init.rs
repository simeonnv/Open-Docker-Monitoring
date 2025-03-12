use bollard::Docker;

use crate::{error::Error, libs::docker::{connect_to_docker::connect_to_docker, get_docker_connections_db::get_docker_connections_db}};
use super::DockerRealtimeConnections;


impl DockerRealtimeConnections {
    pub async fn init(&self) -> Result<(), Error> {
        
        let docker_connections = get_docker_connections_db().await?;

        for docker_connection in docker_connections {
            
            let docker: Docker = match connect_to_docker(&docker_connection).await {
                Err(e) => { 
                    print!("{}", e);
                    continue
                },
                Ok(d) => d
            };
            
            self.insert(docker_connection.name, docker).await;
        } 
        // // ! this could lead to error in the close future
        // // ! if a docker stored from the database failes to load on runtime, it will lead to a whole exeption crash
        // // ! if not fixed can softlock the entire server
    
        Ok(())
    }    
}