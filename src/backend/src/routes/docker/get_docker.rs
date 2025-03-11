use std::collections::HashMap;

use actix_web::{get, HttpMessage, HttpRequest, HttpResponse};
use bollard::system::Version;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use utoipa::ToSchema;

use crate::error::Error;
use crate::libs::auth::auth_middleware::AccountData;
use crate::libs::docker::docker_realtime_connections::REALTIME_CONNECTED_DOCKERS;

#[derive(Serialize, Debug)]
struct Res {
    status: &'static str,
    data: Option<HashMap<String, Version>>
}


#[utoipa::path(
    get,
    path = "/docker",
    responses(
        (status = 200, description = "Signup successful", body = GetFilesIdResDocs, example = json!({
            "status": "success",
            "data":{
                "file_id": 12,
                "file_blob": "BLOB",
                "size": 66666,
                "file_type": "image/png",
                "account_id": 12,
                "created_at": "TIME NOW",
            }
        })),
        (status = 401, description = "Unauthorized", body = GetFilesIdResDocs, example = json!({
            "status": "Invalid premisions",
            "data": ""
        })),
        (status = 400, description = "Bad Request", body = GetFilesIdResDocs, example = json!({
            "status": "Bad request data",
            "data": ""
        }))
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
            data: Some(docker_data),
        }))

    } else {
        return Ok(HttpResponse::Unauthorized().json(Res {
            status: "Unauthorized access",
            data: None,
        }))
    }
}


#[derive(Serialize, ToSchema)]
struct GetFilesIdResDocs {
    status: &'static str,
    data: Option<FilesResDocs>,
}

#[derive(sqlx::FromRow, Debug, Serialize, ToSchema)]
pub struct FilesResDocs {
    pub file_id: i64,
    pub file_blob: Vec<u8>,
    pub size: i64,
    pub file_type: String,
    pub account_id: i64,
    pub created_at: String,
}





