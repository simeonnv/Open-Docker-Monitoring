
use actix_web::web::Query;
use actix_web::{delete, post, web, HttpMessage, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::docker::docker_realtime_connections::REALTIME_CONNECTED_DOCKERS;
use crate::libs::docker::structs::docker_connection::DockerConnection;

#[derive(Serialize, Debug, ToSchema)]
#[schema(as = Post::Docker::Res)]
struct Res {
    status: &'static str,
    data: &'static str
}


#[utoipa::path(
    delete,
    path = "/docker/{name}",
    params(
        ("name" = String, Path, description = "Unique docker name")
    ),
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
#[delete("/{name}")]
async fn delete_docker_name(
    token_data: HttpRequest,
    path: web::Path<String>
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        let docker_name = path.to_string();

        REALTIME_CONNECTED_DOCKERS.remove_docker(docker_name).await?;

        return Ok(HttpResponse::Ok().json(Res {
            status: "success",
            data: "",
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: "",
        }))
    }
}





