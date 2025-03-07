use crate::error::Error;

use super::{connect_to_docker::connect_to_docker, create_docker_background_thread::create_docker_background_thread, get_docker_connections_db::get_docker_connections_db};


pub async fn init_docker_container_threads() -> Result<(), Error> {
    let docker_connections = get_docker_connections_db().await?;
    
    for docker in docker_connections {
        let docker_connection = connect_to_docker(&docker).await?;
        create_docker_background_thread(docker.name, docker_connection).await;
    }

    Ok(())
}