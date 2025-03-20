use bollard::container::ListContainersOptions;
use crate::{error::Error, libs::docker::structs::container_refined_summary::ContainerRefinedSummary};
use super::DockerRealtimeConnections;
use tokio::task;

impl DockerRealtimeConnections {
    pub async fn list_containers_all(&self) -> Result<Vec<ContainerRefinedSummary>, Error> {
        let guard = self.inner.read().await;

        let options = ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        };

        let mut tasks = Vec::new();

        // Spawn a task for each Docker instance to fetch containers concurrently for silly max speed
        for (_, docker) in guard.values() {
            let docker = docker.clone();
            let options = options.clone();

            let task = task::spawn(async move {
                docker.list_containers(Some(options)).await
            });

            tasks.push(task);
        }

        let mut all_containers = Vec::new();

        // now join that crap together
        for task in tasks {
            match task.await {
                Ok(Ok(mut containers)) => all_containers.append(&mut containers),
                Ok(Err(e)) => {
                    eprintln!("Error listing containers: {}", e);
                }
                Err(e) => {
                    eprintln!("Task failed: {}", e);
                }
            }
        }

        let all_refined_containers = all_containers.into_iter().map(|e| {
            e.into()
        }).collect();

        Ok(all_refined_containers)
    }
}