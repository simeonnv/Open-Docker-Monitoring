use crate::{error::Error, libs::db::get_pool::get_pool};



pub async fn check_local_connection_exist() -> Result<(), Error> {

    let pool = get_pool();

    let exist: bool = sqlx::query_scalar(r#"
    
        SELECT COUNT(*) FROM DockerConnections WHERE protocol = 'local'
    
    "#)
    .fetch_one(pool)
    .await?;

    match exist {
        true =>  Err(Error::Conflict("Local connection already exists!".to_string())),
        false => Ok(())
    }

}