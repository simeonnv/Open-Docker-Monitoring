
use actix_web::{dev::{ServiceFactory, ServiceRequest, ServiceResponse}, Error, web, Scope};
use crate::libs::auth::auth_middleware::AuthMiddleware;

// pub mod post_image_id;
pub mod get_docker;
pub mod post_docker;
pub mod delete_docker_name;

pub fn docker() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse, Error = Error, InitError = ()>> {
    web::scope("/docker")
        .wrap(AuthMiddleware)

        .service(get_docker::get_docker)
        .service(post_docker::post_docker)
        .service(delete_docker_name::delete_docker_name)
}
