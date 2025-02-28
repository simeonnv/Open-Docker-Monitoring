use actix_web::{web, Scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_docs;

pub mod files;
pub mod docker;
pub mod auth;


pub fn routes() -> Scope {
    web::scope("")

        .service(files::files())
        .service(docker::docker())
        .service(auth::auth())
        .service(SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", api_docs::ApiDoc::openapi()))
        
}
