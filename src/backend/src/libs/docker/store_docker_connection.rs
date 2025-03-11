use crate::{error::Error, libs::{db::get_pool::get_pool, util::validate_protocol::validate_protocol}};

use super::{check_local_connection_exist_db::check_local_connection_exist_db, structs::docker_connection::DockerConnection};



pub async fn store_docker_connection(docker_connection: &DockerConnection) -> Result<(), Error> {

    let pool = get_pool();

    validate_protocol(&docker_connection.protocol)?;

    if docker_connection.protocol == "local" {
        check_local_connection_exist_db().await?;
    }

    sqlx::query(r#"
    
        INSERT INTO DockerConnections (name, host, protocol)
        VALUES ($1, $2, $3)
    
    "#)
        .bind(docker_connection.name.clone())
        .bind(docker_connection.host.clone())
        .bind(docker_connection.protocol.clone())
        .execute(pool)
        .await?;

    Ok(())    
}