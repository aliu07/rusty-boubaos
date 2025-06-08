use crate::errors::BadCommandError;
use std::{env, path::Path};

pub fn cd(target_path: &str) -> Result<(), BadCommandError> {
    // Build new path
    let target_path = Path::new(target_path);
    let mut path_buf =
        env::current_dir().map_err(|_| BadCommandError::CurrentDirectoryReadError)?;
    path_buf.push(target_path);

    env::set_current_dir(&path_buf)
        .map_err(move |_| BadCommandError::PathDoesNotExist(path_buf.display().to_string()))?;

    Ok(())
}
