use crate::{error::Error, libs::{db::get_pool::get_pool, util::validate_protocol::validate_protocol}};

use super::check_local_connection_exist_db::check_local_connection_exist_db;



pub async fn store_docker_connection(name: String, host: String, protocol: String, cert_path: Option<String>) -> Result<(), Error> {

    let pool = get_pool();

    validate_protocol(&protocol)?;

    if protocol == "local" {
        check_local_connection_exist_db().await?;
    }

    sqlx::query(r#"
    
        INSERT INTO DockerConnections (name, host, protocol, cert_path)
        VALUES ($1, $2, $3, $4)
    
    "#)
        .bind(name)
        .bind(host)
        .bind(protocol)
        .bind(cert_path)
        .execute(pool)
        .await?;

    Ok(())    
}