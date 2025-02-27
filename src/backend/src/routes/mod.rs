use actix_web::{web, Scope};

pub mod files;
pub mod docker;
pub mod auth;


pub fn routes() -> Scope {
    web::scope("")

        .service(files::files())
        .service(docker::docker())
        .service(auth::auth())
        
}
