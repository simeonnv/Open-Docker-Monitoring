use bollard::{container::ListContainersOptions, secret::ContainerSummary, Docker};
use crate::error::Error;

pub async fn list_containers_for_docker(docker: &Docker) -> Result<Vec<ContainerSummary>, Error> {
    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };
    Ok(docker.list_containers(Some(options)).await?)
}