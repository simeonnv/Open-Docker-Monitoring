use argon2::{
    password_hash::{
        Error, PasswordHash, PasswordVerifier
    }, Algorithm, Argon2, Version
};

use crate::env;

pub async fn compare(input: &String, hash: &String) -> Result<bool, Error> {

    let argon2 = Argon2::new(
        Algorithm::Argon2id, 
        Version::V0x13,      
        env::ARGON2_PARAMS?,              
    );

    let parsed_hash = PasswordHash::new(hash)?;

    let is_correct = argon2.verify_password(input.as_bytes(), &parsed_hash).is_ok();

    Ok(is_correct)
}