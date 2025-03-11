use crate::{error::Error, libs::db::get_pool::get_pool};
use super::structs::docker_connection::DockerConnection;

pub async fn get_docker_connections_db() -> Result<Vec<DockerConnection>, Error> {

    let pool = get_pool();

    let docker_connections: Vec<DockerConnection> = sqlx::query_as(r#"
    
        SELECT name, host, protocol FROM DockerConnections;

    "#)
        .fetch_all(pool)
        .await?;

    Ok(docker_connections)    
}