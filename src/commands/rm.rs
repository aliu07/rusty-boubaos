use std::{env, fs, path::Path};

use crate::errors::BadCommandError;

pub fn rm(file_name: &str) -> Result<(), BadCommandError> {
    let mut path_buf =
        env::current_dir().map_err(|_| BadCommandError::CurrentDirectoryReadError)?;
    path_buf.push(Path::new(&file_name));

    match fs::remove_file(path_buf) {
        Ok(_) => Ok(()),
        Err(_) => Err(BadCommandError::FileNotFound(String::from(file_name))),
    }
}
