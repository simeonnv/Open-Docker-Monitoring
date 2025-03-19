use crate::{error::Error, libs::db::get_pool::get_pool};


pub async fn does_docker_exist_db(name: &String) -> Result<bool, Error> {
    
    let pool = get_pool();

    let connection_exists: bool =sqlx::query_scalar(r#"
        
        SELECT EXISTS (
            SELECT 1 FROM DockerConnections WHERE name = $1
        );

    "#)
        .bind(name)
        .fetch_one(pool)
        .await?;

    Ok(connection_exists)
}