use std::env;
use argon2::Params;
use dotenv::from_filename;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Env {
    pub port: u16,
    pub listening_on: String,
    pub db_port: u16,
    pub db_address: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,

    pub init_username: String,
    pub init_password: String
}

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Idk what is this tbh
);

pub fn load_config() -> Env {
    from_filename("../../.env").ok();

    Env {
        port: env::var("PORT")
            .unwrap_or("6004".to_string())
            .parse::<u16>()
            .unwrap_or(6004),
        listening_on: env::var("LISTENING_ON").unwrap_or("0.0.0.0".to_string()),
        db_port: env::var("DB_PORT")
            .unwrap_or("5432".to_string())
            .parse::<u16>()
            .unwrap_or(5432),
        db_address: env::var("DB_ADDRESS").unwrap_or("odm_database".to_string()),
        db_name: env::var("DB_NAME").unwrap_or("my_database".to_string()),
        db_username: env::var("DB_USERNAME").unwrap_or("postgres".to_string()),
        db_password: env::var("DB_PASSWORD").unwrap_or("root".to_string()),
        init_username: env::var("INIT_PASSWORD").unwrap_or("root".to_string()),
        init_password: env::var("INIT_PASSWORD").unwrap_or("root".to_string()),
    }
}

// Optional: Getter for convenience if you don’t want to call load_config() everywhere
lazy_static! {
    pub static ref ENV: Env = load_config();
}