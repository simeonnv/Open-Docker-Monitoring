use bollard::Docker;
use crate::error::Error;
use super::get_docker_connections_db::DockerConnection;
use bollard::ClientVersion;

pub async fn connect_to_docker(docker_connection: &DockerConnection) -> Result<Docker, Error> {
    // Define the client version - typically something like "1.41" or "1.42"
    let client_version = ClientVersion { major_version: 1, minor_version: 41 }; // Adjust version as needed
    
    let docker = match docker_connection.protocol.as_str() {
        "http" => Docker::connect_with_http(
            &docker_connection.host, 
            3000, 
            &client_version
        ),
        "ssl" => Docker::connect_with_ssl_defaults(), // Using defaults for SSL
        "local" => Docker::connect_with_local_defaults(),
        _ => return Err(Error::Conflict("Invalid protocol".to_string()))
    }?;

    Ok(docker)
}