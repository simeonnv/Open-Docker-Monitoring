use crate::{error::Error, libs::db::get_pool::get_pool};

#[derive(sqlx::FromRow, Debug)]
pub struct DockerConnection {
    pub name: String,
    pub host: String,
    pub protocol: String,
    pub cert_path: Option<String>
}

pub async fn get_docker_connections_db() -> Result<Vec<DockerConnection>, Error> {

    let pool = get_pool();

    let docker_connections: Vec<DockerConnection> = sqlx::query_as(r#"
    
        SELECT name, host, protocol, cert_path FROM DockerConnections;

    "#)
        .fetch_all(pool)
        .await?;

    Ok(docker_connections)    
}