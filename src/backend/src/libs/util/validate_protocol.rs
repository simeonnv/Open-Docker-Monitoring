use crate::error::Error;

pub fn validate_protocol(protocol: &str) -> Result<(), Error> {
    match protocol {
        "http" => Ok(()),
        "local" => Ok(()),
        _ => Err(Error::Conflict("Invalid protocol".to_string()))
    }
}