use std::fs;

use crate::ENV;
use crate::errors::BadCommandError;

pub fn rm(file_name: &str) -> Result<(), BadCommandError> {
    let mut path = ENV.get_current_path();
    path.push(file_name);

    match fs::remove_file(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(BadCommandError::FileNotFound(String::from(file_name))),
    }
}
