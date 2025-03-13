// use bollard::ClientVersion;

use serde::Serialize;
use utoipa::ToSchema;



#[derive(sqlx::FromRow, Debug, Clone, Serialize, ToSchema)]
pub struct DockerConnection {
    pub name: String,
    pub host: String,
    pub protocol: String,
    // pub version: ClientVersion
}