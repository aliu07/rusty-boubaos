use std::env;

use crate::errors::BadCommandError;

pub fn pwd() -> Result<(), BadCommandError> {
    let path_buf = env::current_dir().map_err(|_| BadCommandError::CurrentDirectoryReadError)?;
    println!("{}", path_buf.display());
    Ok(())
}
