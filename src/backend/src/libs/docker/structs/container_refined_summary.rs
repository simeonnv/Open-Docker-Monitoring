use bollard::secret::{ContainerSummary, PortTypeEnum};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ContainerRefinedSummary {
    pub id: String,
    pub names: Vec<String>,
    pub image_id: String,
    pub command: String,
    pub created: String,
    pub ports: Vec<String>,
    pub state: String,
    pub status: String,
    pub network: String,
}

impl From<ContainerSummary> for ContainerRefinedSummary {
    fn from(raw_info: ContainerSummary) -> Self {

        let ports = raw_info.ports.unwrap_or_default()
            .into_iter()
            .map(|port| {
                format!(
                    "{}:{}/{} -> {}:{}",
                    port.ip.clone().unwrap_or_default(),
                    port.private_port,
                    port.typ.unwrap_or(PortTypeEnum::EMPTY),
                    port.ip.unwrap_or_default(),
                    port.public_port.unwrap_or_default()
                )
            })
            .collect();


        ContainerRefinedSummary {
            id: raw_info.id.unwrap_or("the id couldnt be fetched :(".to_string()),
            names: raw_info.names.unwrap_or_default(),
            image_id: raw_info.image_id.unwrap_or("the id couldnt be fetched :(".to_string()),
            command: raw_info.command.unwrap_or("Couldnt fetch the init command".to_string()),
            created: DateTime::from_timestamp(raw_info.created.unwrap_or_default(), 0).unwrap_or_default().format("%Y-%m-%d %H:%M:%S").to_string(),
            ports: ports,
            state: raw_info.state.unwrap_or("unknown state!".to_string()),
            status: raw_info.status.unwrap_or("unknown status!".to_string()),
            network: raw_info.host_config.unwrap_or_default().network_mode.unwrap_or("unknown network!".to_string()),
        }
    }
}
