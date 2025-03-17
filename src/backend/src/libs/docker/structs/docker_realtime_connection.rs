use serde::Serialize;
use utoipa::ToSchema;

use super::docker_connection::DockerConnection;

#[derive(sqlx::FromRow, Debug, Clone, Serialize, ToSchema)]
pub struct DockerRealtimeConnection {
    pub name: String,
    pub host: String,
    pub protocol: String,
    pub connected: bool,
    pub connection_error: Option<String>
}

impl From<(DockerConnection, bool, Option<String>)> for DockerRealtimeConnection {
    fn from((dc, connected, connection_error): (DockerConnection, bool, Option<String>)) -> Self {
        DockerRealtimeConnection {
            name: dc.name,
            host: dc.host,
            protocol: dc.protocol,
            connected,
            connection_error,
        }
    }
}