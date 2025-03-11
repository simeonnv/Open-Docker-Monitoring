use crate::{error::Error, libs::{docker::{connect_to_docker::connect_to_docker, store_docker_connection::store_docker_connection, structs::docker_connection::DockerConnection}, util::validate_protocol::validate_protocol}};
use super::DockerRealtimeConnections;


impl DockerRealtimeConnections {
    pub async fn new_docker(&self, docker_connection: DockerConnection) -> Result<(), Error> {
            
        validate_protocol(&docker_connection.protocol)?;
    
        let docker = connect_to_docker(&docker_connection).await?;
        
        store_docker_connection(&docker_connection).await?;
        
        self.insert(docker_connection.name.clone(), docker).await?;
        
        Ok(())    

    }    
}