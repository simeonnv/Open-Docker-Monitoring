use bollard::Docker;
use crate::error::Error;
use bollard::ClientVersion;
use super::structs::docker_connection::DockerConnection;

pub async fn connect_to_docker(docker_connection: &DockerConnection) -> Result<Docker, Error> {
    let client_version = ClientVersion { major_version: 1, minor_version: 48 }; // todo version prop
    
    let docker = match docker_connection.protocol.as_str() {
        "http" => Docker::connect_with_http(
            &docker_connection.host, 
            3000, 
            &client_version
        ), // Using defaults for SSL
        "local" => Docker::connect_with_local_defaults(),
        _ => return Err(Error::Conflict("Invalid protocol".to_string()))
    }?;

    Ok(docker)
}