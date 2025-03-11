
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::{env::ENV, error::Error, libs::{auth::{does_account_exist::does_account_exist, key}, crypto::rand_string::rand_string}};

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Key::Req)]
pub struct Req {
    pub username: String,
    pub password: String,
}


#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = Post::Auth::Signup::Key::Res)]
struct Res {
    status: &'static str,
    data: String
}

#[utoipa::path(
    post,
    path = "/auth/signup_key",
    request_body = Req,
    description = "For one to signup. first you need to get a signup key. To get a signup key you need to login first using your ENV variable credentials (autism)",
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
