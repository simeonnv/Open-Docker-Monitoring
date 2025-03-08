use crate::error::Error;

use super::{connect_to_docker::connect_to_docker, get_docker_connections_db::get_docker_connections_db, REALTIME_CONNECTED_DOCKERS};


pub async fn init_dockers() -> Result<(), Error> {

    let docker_connections = get_docker_connections_db().await?;

    for docker_connection in docker_connections {
        let docker = connect_to_docker(&docker_connection).await?;
        REALTIME_CONNECTED_DOCKERS.write().await.insert(docker_connection.name, docker);
    }

    Ok(())
}