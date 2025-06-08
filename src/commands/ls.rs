use std::{env, fs};

use crate::errors::BadCommandError;

pub fn ls() -> Result<(), BadCommandError> {
    let path_buf = env::current_dir().map_err(|_| BadCommandError::CurrentDirectoryReadError)?;
    let path = path_buf.as_path();
    let mut entries: Vec<String> = Vec::new();

    for entry in fs::read_dir(path).map_err(|_| BadCommandError::CurrentDirectoryReadError)? {
        let entry = entry.map_err(|_| BadCommandError::DirectoryEntryReadError)?;

        if let Some(file_name) = entry.file_name().to_str() {
            entries.push(file_name.to_owned());
        }
    }

    entries.sort();

    for entry in entries {
        println!("{}", entry);
    }

    Ok(())
}
