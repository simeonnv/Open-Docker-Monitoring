use sqlx::{Sqlite, Pool};

use crate::DB;

pub fn get_pool() -> &'static Pool<Sqlite> {
    DB.get().expect("Database pool is not initialized")
}