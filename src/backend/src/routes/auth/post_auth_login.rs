
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::error::Error;
use crate::libs::auth::check_credentials::check_credentials;
use crate::libs::auth::create_token::create_token;
use crate::libs::util::insure_len::insure_len;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Login::Req)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Login::Res)]
struct Res {
    status: &'static str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = Req,
    responses(
        (status = 200, description = "Login successful", body = Res, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = Res, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = Res, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/login")]
pub async fn post_auth_login(req: web::Json<Req>) -> Result<HttpResponse, Error> {

    insure_len(&req.username, 5, 15)?;
    insure_len(&req.password, 5, 30)?;

    let account = check_credentials(&req.username, &req.password).await?;

    let token: String = create_token(&account.0, account.1).await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: token,
    }));
   
}

