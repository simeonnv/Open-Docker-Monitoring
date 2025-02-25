use sqlx::{Postgres, Pool};
use sqlx::postgres::PgPoolOptions;
use crate::env::ENV;


pub async fn init_db() -> Result<Pool<Postgres>, sqlx::Error> {
    println!("Connecting to db");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://{}:{}@{}", ENV.db_username, ENV.db_password, ENV.db_address).as_str())
        .await?;

    println!("Connected to postgres://{}:{}@{}", ENV.db_username, ENV.db_password, ENV.db_address);

    let _ = sqlx::query(&format!("CREATE DATABASE {}",ENV.db_name))
        .execute(&pool)
        .await;

    println!("Database '{}' created or already exists!", ENV.db_name);

    let pool_with_db: Pool<Postgres> = PgPoolOptions::new()
        .connect(
            format!("postgres://{}:{}@{}/{}", 
                ENV.db_username, 
                ENV.db_password, 
                ENV.db_address,
                ENV.db_name
            ).as_str())
        .await?;

    println!("Connected to db {}", ENV.db_name);

    Ok(pool_with_db)
}


