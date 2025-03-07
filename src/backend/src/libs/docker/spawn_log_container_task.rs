use std::{collections::HashSet, sync::Arc, time::SystemTime};
use bollard::{container::{LogOutput, LogsOptions}, Docker};
use tokio::sync::Mutex;
use crate::libs::db::get_pool::get_pool;
use futures_util::stream::StreamExt;

pub fn spawn_log_container_task(
    docker: Docker,
    container_id: String,
    active_containers: Arc<Mutex<HashSet<String>>>,
    docker_name: String,
) {
    let logs = docker.logs(&container_id, Some(LogsOptions::<String> {
        follow: true,
        stderr: true,
        stdout: false,
        timestamps: true,
        ..Default::default()
    }));
    let pool = get_pool();
    let active_clone = active_containers.clone();

    tokio::spawn(async move {
        let mut logs = logs;
        while let Some(Ok(log)) = logs.next().await {
            if let LogOutput::StdErr { message } = log {
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string();
                let error_message = String::from_utf8_lossy(&message).to_string();

                if let Err(e) = sqlx::query(
                    "INSERT INTO container_logs (docker_name, container_id, timestamp, error_message) VALUES (?, ?, ?, ?)"
                )
                .bind(&docker_name)
                .bind(&container_id)
                .bind(&timestamp)  // Fixed typo from ×tamp
                .bind(&error_message)
                .execute(pool)
                .await
                {
                    println!("DB insert error: {}", e);
                }
            }
        }
        active_clone.lock().await.remove(&container_id);
    });
}