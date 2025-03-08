use crate::{error::Error, libs::util::validate_protocol::validate_protocol};

use super::{connect_to_docker::connect_to_docker, store_docker_connection::store_docker_connection, structs::docker_connection::DockerConnection, REALTIME_CONNECTED_DOCKERS};



pub async fn add_new_docker(docker_connection: DockerConnection) -> Result<(), Error> {
    validate_protocol(&docker_connection.protocol)?;
    
    let docker = connect_to_docker(&docker_connection).await?;

    store_docker_connection(docker_connection.name.clone(), docker_connection.host, docker_connection.protocol, docker_connection.cert_path).await?;

    REALTIME_CONNECTED_DOCKERS.write().await.insert(docker_connection.name.clone(), docker);

    Ok(())    
}