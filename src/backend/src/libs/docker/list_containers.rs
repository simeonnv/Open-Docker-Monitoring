use bollard::{container::ListContainersOptions, secret::ContainerSummary, Docker};

pub async fn list_containers(docker: &Docker) -> Result<Vec<ContainerSummary>, bollard::errors::Error> {
    let options = ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    };
    docker.list_containers(Some(options)).await
}