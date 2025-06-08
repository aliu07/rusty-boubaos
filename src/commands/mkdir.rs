use std::fs::DirBuilder;

use crate::errors::BadCommandError;

pub fn mkdir(dirname: &str) -> Result<(), BadCommandError> {
    DirBuilder::new()
        .create(dirname)
        .map_err(|_| BadCommandError::DirectoryAlreadyExists(String::from(dirname)))?;

    Ok(())
}
