use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;
use crate::env::ENV;

pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    println!("Connecting to db");

    // Connection string for initial connection (without DB name)
    let conn_str = format!(
        "postgres://{}:{}@{}:{}",
        ENV.db_username, ENV.db_password, ENV.db_address, ENV.db_port
    );
    println!("Attempting connection: {}", conn_str);

    // Initial connection to the PostgreSQL server
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&conn_str)
        .await?;

    println!(
        "Connected to postgres://{}:{}@{}:{}",
        ENV.db_username, ENV.db_password, ENV.db_address, ENV.db_port
    );

    // Attempt to create the database (ignore if it already exists)
    let _ = sqlx::query(&format!("CREATE DATABASE {}", ENV.db_name))
        .execute(&pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("already exists") {
                println!("Database '{}' already exists, proceeding...", ENV.db_name);
            } else {
                panic!("Failed to create database: {}", e);
            }
        });

    // Connection string for the specific database
    let db_conn_str = format!(
        "postgres://{}:{}@{}:{}/{}",
        ENV.db_username, ENV.db_password, ENV.db_address, ENV.db_port, ENV.db_name
    );
    println!("Attempting connection to db: {}", db_conn_str);

    // Connect to the specific database
    let pool_with_db: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_conn_str)
        .await?;

    println!("Connected to db {}", ENV.db_name);

    Ok(pool_with_db)
}