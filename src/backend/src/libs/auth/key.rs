use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::error::Error;

lazy_static! {
    pub static ref KEY: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

pub fn compare(provided_key: &str) -> Result<bool, Error> {
    let key = KEY.lock()?;
    Ok(*key == provided_key)
}

pub fn set(new_key: String) ->  Result<bool, Error> {
    let mut key = KEY.lock()?; // Get the lock and unwrap (panics on poison)
    *key = new_key;
    Ok(true)
}