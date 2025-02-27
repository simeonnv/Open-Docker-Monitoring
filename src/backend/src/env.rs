use std::env;
use argon2::Params;
use lazy_static::lazy_static;
use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Env {
    pub backend_port: u16,
    pub db_port: u16,
    pub db_address: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
    pub init_username: String,
    pub init_password: String,
}

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Optional output length (None uses default)
);

pub fn load_config() -> Env {

    dotenv().ok();

    Env {
        backend_port: env::var("BACKEND_PORT")
            .unwrap_or_else(|_| "6004".to_string())
            .parse::<u16>()
            .unwrap_or(6004),
        db_port: env::var("POSTGRES_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse::<u16>()
            .unwrap_or(5432),
        db_address: env::var("POSTGRES_ADDRESS")
            .unwrap_or_else(|_| "odm_database".to_string()),
        db_name: env::var("POSTGRES_DATABASE")
            .unwrap_or_else(|_| "my_database".to_string()),
        db_username: env::var("POSTGRES_USER")
            .unwrap_or_else(|_| "postgres".to_string()),
        db_password: env::var("POSTGRES_PASSWORD")
            .unwrap_or_else(|_| "root".to_string()),
        init_username: env::var("INIT_USERNAME")
            .unwrap_or_else(|_| "root".to_string()),
        init_password: env::var("INIT_PASSWORD")
            .unwrap_or_else(|_| "root".to_string()),
    }
}

lazy_static! {
    pub static ref ENV: Env = load_config();
}