use std::env;

use crate::errors::BadCommandError;

pub fn pwd() -> Result<(), BadCommandError> {
    let path = env::current_dir().map_err(|_| BadCommandError::PwdError)?;
    println!("{}", path.display());
    Ok(())
}
