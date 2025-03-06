use std::{collections::HashSet, sync::Arc, thread, time::Duration};
use tokio::{sync::Mutex, time::sleep};

use crate::RT;

use super::{is_docker_active::is_docker_active, list_containers::list_containers, spawn_log_container_task::spawn_log_container_task, REALTIME_CONNECTED_DOCKERS};

pub fn create_docker_listiner_worker(docker_name: String) {
    let dockers = REALTIME_CONNECTED_DOCKERS.clone();
    let active_containers = Arc::new(Mutex::new(HashSet::new()));

    thread::spawn(move || {
        RT.block_on(async {
            loop {
                if !is_docker_active(&docker_name).await {
                    println!("Docker {} removed, stopping listener", docker_name);
                    break;
                }

                let docker = {
                    let dockers = dockers.lock().await;
                    match dockers.get(&docker_name) {
                        Some(e) => e.clone(),
                        None => break
                    }
                };

                if let Ok(containers) = list_containers(&docker).await {
                    let mut active = active_containers.lock().await;
                    let current_ids: HashSet<_> = containers.iter()
                        .map(|c| c.id.clone().unwrap_or_default())
                        .collect();
                    active.retain(|id| current_ids.contains(id));

                    for container in containers {
                        let container_id = container.id.unwrap_or_default();
                        if !active.contains(&container_id) {
                            active.insert(container_id.clone());
                            spawn_log_container_task(
                                docker.clone(),
                                container_id,
                                active_containers.clone(),
                                docker_name.clone(),
                            );
                        }
                    }
                }

                sleep(Duration::from_secs(5)).await;
            }
        });
    });
}