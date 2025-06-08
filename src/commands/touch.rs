use crate::errors::BadCommandError;
use std::fs::File;

pub fn touch(file_name: &str) -> Result<(), BadCommandError> {
    File::create(file_name).map_err(|_| BadCommandError::CreateFileError)?;

    Ok(())
}
