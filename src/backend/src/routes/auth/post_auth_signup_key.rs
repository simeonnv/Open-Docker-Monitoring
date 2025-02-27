
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{env::{Env, ENV}, error::Error, libs::{auth::{create_account::create_account, create_token::create_token, does_account_exist::does_account_exist, key}, crypto::rand_string::rand_string, util::insure_len::insure_len}};

#[derive(Serialize, Deserialize)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize)]
struct Res {
    status: &'static str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/signup_key",
    request_body = PostAuthSignupDocReq,
    description = "For one to signup. first you need to get a signup key. To get a signup key you need to login first using your ENV variable credentials (autism)",
    responses(
        (status = 200, description = "Signup successful", body = PostAuthSignupDocsRes, example = json!({
            "status": "success",
            "token": "abc123xyz456"
        })),
        (status = 401, description = "Unauthorized", body = PostAuthSignupDocsRes, example = json!({
            "status": "incorrect credential",
            "token": ""
        })),
        (status = 409, description = "Conflict", body = PostAuthSignupDocsRes, example = json!({
            "status": "account already exists",
            "token": ""
        }))
    ),
    security(),
    tag = "Auth"
)]
#[post("/signup_key")]
pub async fn post_auth_signup_key(req: web::Json<Req>) -> Result<HttpResponse, Error> {
  
    if does_account_exist().await? {
        return Err(Error::Unauthorized("A main account already exists!".to_string()))
    }

    if (req.username != ENV.init_username) && (req.password != ENV.init_password) {
        return Err(Error::Unauthorized("Invalid credentials".to_string()))
    }


    let key = rand_string(10);

    key::set(key.clone())?;

    return Ok(HttpResponse::Ok().json(Res {
        status: "success",
        data: key,
    }));
   
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(title = "Signup Request")]
pub struct PostAuthSignupDocReq {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[schema(title = "Signup Response")]
struct PostAuthSignupDocsRes {
    status: String,
    token: String
}