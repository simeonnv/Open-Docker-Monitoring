use std::collections::HashMap;

use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::docker::docker_realtime_connections::REALTIME_CONNECTED_DOCKERS;
use crate::libs::docker::structs::docker_connection::DockerConnection;
use crate::libs::docker::structs::docker_info::DockerInfo;

#[derive(Serialize, Debug, ToSchema)]
#[schema(as = Get::Docker::Res)]
struct Res {
    status: &'static str,
    data: HashMap<String, (DockerConnection, DockerInfo)>
}


#[utoipa::path(
    get,
    path = "/docker",
    responses(
        (status = 200, description = "Successful", body = Res),
        (status = 401, description = "Unauthorized", body = Res),
        (status = 400, description = "Bad Request", body = Res)
    ),
    security(
        ("bearer_auth" = [])
    ),
    tag = "Docker"
)]
#[get("")]
async fn get_docker(
    token_data: HttpRequest,
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let docker_data = REALTIME_CONNECTED_DOCKERS.list_dockers().await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "success",
            data: docker_data,
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: HashMap::new(),
        }))
    }
}





