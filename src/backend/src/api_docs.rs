use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use crate::routes;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};


#[derive(Serialize, Deserialize, ToSchema)]
pub struct Req {
    pub username: String,
    pub password: String,
}

struct BearerAuthAddon;

impl Modify for BearerAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}


#[derive(OpenApi)]
#[openapi(
    paths(
        routes::auth::post_auth_signup::post_auth_signup,
        routes::auth::post_auth_login::post_auth_login,
        routes::auth::get_auth_validate::get_auth_validate,
        routes::auth::get_auth_exists::get_auth_exists,

        routes::files::get_files_id::get_files_id,
        routes::files::post_files::post_files,

        routes::docker::get_docker::get_docker,
        routes::docker::post_docker::post_docker,
        routes::docker::delete_docker_name::delete_docker_name,
        routes::docker::get_docker_containers::get_docker_containers
        
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        // (name = "Users", description = "User management endpoints")
    ),
    modifiers(&BearerAuthAddon),
    security(
        ("bearer_auth" = [])
    )
   
)]
pub struct ApiDoc;