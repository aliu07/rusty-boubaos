use std::fs;

use crate::ENV;
use crate::errors::BadCommandError;

pub fn rmdir(dir_name: &str) -> Result<(), BadCommandError> {
    let mut path = ENV.get_current_path();
    path.push(dir_name);

    match fs::remove_dir(path) {
        Ok(_) => Ok(()),
        Err(_) => Err(BadCommandError::DirectoryNotFound(String::from(dir_name))),
    }
}
