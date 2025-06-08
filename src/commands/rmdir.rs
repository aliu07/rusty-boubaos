use std::{env, fs, path::Path};

use crate::errors::BadCommandError;

pub fn rmdir(dir_name: &str) -> Result<(), BadCommandError> {
    let mut path_buf =
        env::current_dir().map_err(|_| BadCommandError::CurrentDirectoryReadError)?;
    path_buf.push(Path::new(&dir_name));

    match fs::remove_dir(path_buf) {
        Ok(_) => Ok(()),
        Err(_) => Err(BadCommandError::DirectoryNotFound(String::from(dir_name))),
    }
}
