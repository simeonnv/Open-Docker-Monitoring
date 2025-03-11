
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{error::Error, libs::{auth::{create_account::create_account, create_token::create_token, does_account_exist::does_account_exist, key}, util::insure_len::insure_len}};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Req)]
pub struct Req {
    pub username: String,
    pub password: String,
    pub key: String
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Res)]
struct Res {
    status: &'static str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/signup",
    request_body = Req,
    responses(
        (status = 200, description = "Signup successful", body = Res, example = json!({
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
#[post("/signup")]
pub async fn post_auth_signup(req: web::Json<Req>) -> Result<HttpResponse, Error> {

    insure_len(&req.username, 5, 15)?;
    insure_len(&req.password, 5, 30)?;
    
    if does_account_exist().await? {
        return Err(Error::Unauthorized("A main account already exists!".to_string()))
    }

    dbg!(&*key::KEY);
    dbg!(&req.key);

    if !key::compare(&req.key)? {
        return Err(Error::Unauthorized("invalid signup key!".to_string()))
    }

    let account_id = create_account(&req.username, &req.password, "admin").await?;

    let token: String = create_token(&account_id, "admin".to_string()).await?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: token,
    }));
   
}

