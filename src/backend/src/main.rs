
use actix_cors::Cors;
use env::ENV;

use libs::{auth::create_account::create_account, db, docker::docker_realtime_connections::REALTIME_CONNECTED_DOCKERS};
use routes::routes;
use tokio::{runtime::Runtime, sync::OnceCell};

use actix_web::{middleware::Logger, web::PayloadConfig, App, HttpServer};
use env_logger::Env;
use sqlx::{Pool, Sqlite};

pub mod api_docs;
pub mod env;
pub mod error;
pub mod libs;
pub mod routes;

static DB: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

    
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    db::init_pool::init_pool().await.expect("Failed to initialize database");
    db::init_tables::init_tables().await.expect("Failed to initialize tables");

    REALTIME_CONNECTED_DOCKERS.init().await.expect("failed to init dockers!!!!");
    
    let _ = create_account(&"admin".to_string(), &"admin".to_string(), "admin").await;
    
    HttpServer::new(|| {
                
        let cors = Cors::default()
            .allowed_origin("http://localhost:7004") // Explicitly allow Nuxt frontend origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials() // If you're using cookies or auth tokens
            .max_age(3600);
                

        App::new()
            .wrap(cors)

            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            
            .app_data(PayloadConfig::new(64 * 1024 * 1024)) // the max upload is 32mb the voices

            .service(routes())

    })
    .bind(("0.0.0.0", ENV.backend_port))?
    .run()
    .await
}