use crate::settings::Settings;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn read_settings(setting_path: &str) -> Result<Vec<Settings>, Error> {
    let set_string = fs::read_to_string(setting_path)?;

    let result = serde_json::from_str(&set_string);
    match result {
        Ok(settings) => Ok(settings),
        Err(e) => Err(Error::new(ErrorKind::Other, e)),
    }
}