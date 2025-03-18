use crate::{error::Error, libs::db::get_pool::get_pool};


pub async fn remove_docker_connection_db(name: &String) -> Result<(), Error> {
    
    let pool = get_pool();

    let connection_exists=sqlx::query(r#"
    
        DELETE FROM DockerConnections WHERE name = $1;
    
    "#)
        .bind(name)
        .execute(pool)
        .await;

    match connection_exists {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::Conflict(format!("Connection couldnt be deleted: {}", e)))
    }
}