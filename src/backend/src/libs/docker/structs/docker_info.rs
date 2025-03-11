use bollard::secret::SystemInfo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct DockerInfo {
    pub id: Option<String>,
    pub server_version: Option<String>,   
    pub containers: Option<i64>,
    pub containers_running: Option<i64>,
    pub containers_paused: Option<i64>,
    pub containers_stopped: Option<i64>,
    pub images: Option<i64>,
    pub kernel_version: Option<String>,
    pub os_type: Option<String>,
    pub architecture: Option<String>,
    pub ncpu: Option<i64>,
    pub mem_total: Option<i64>,
}

impl From<SystemInfo> for DockerInfo {
    fn from(raw_info: SystemInfo) -> Self {
        DockerInfo {
            id: raw_info.id,
            server_version: raw_info.server_version,   
            containers: raw_info.containers,
            containers_running: raw_info.containers_running,
            containers_paused: raw_info.containers_paused,
            containers_stopped: raw_info.containers_stopped,
            images: raw_info.images,
            kernel_version: raw_info.kernel_version,
            os_type: raw_info.os_type,
            architecture: raw_info.architecture,
            ncpu: raw_info.ncpu,
            mem_total: raw_info.mem_total,
        }
    }
}