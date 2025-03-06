use super::REALTIME_CONNECTED_DOCKERS;

pub async fn is_docker_active(docker_name: &str) -> bool {
    let dockers = REALTIME_CONNECTED_DOCKERS.read().await;
    dockers.contains_key(docker_name)
}