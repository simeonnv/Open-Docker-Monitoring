
use actix_web::{post, web, HttpMessage, HttpRequest, HttpResponse};
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


#[derive(Deserialize, ToSchema)]
#[schema(as = Post::Files::Req)]
pub struct Req {
    pub name: String,
    pub host: String,
    pub protocol: String,    
}


#[utoipa::path(
    post,
    path = "/docker",
    request_body = Req,
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
#[post("")]
async fn post_docker(
    token_data: HttpRequest,
    req: web::Json<Req>
) -> Result<HttpResponse, Error> {
    if let Some(_) = token_data.extensions().get::<AccountData>() {

        REALTIME_CONNECTED_DOCKERS.new_docker(DockerConnection {
            name: req.name.clone(),
            host: req.host.clone(),
            protocol: req.protocol.clone()
        }).await?;

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





