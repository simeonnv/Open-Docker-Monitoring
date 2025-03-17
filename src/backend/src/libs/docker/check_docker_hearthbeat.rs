use bollard::Docker;
use crate::error::Error;


pub async fn check_docker_hearthbeat(docker: &Docker) -> Result<(), Error> {

    match docker.info().await {
        Err(e) => Err(Error::Conflict(format!("Couldnt connect to docker: {}", e))),
        Ok(_) => Ok(())
    }
    
}