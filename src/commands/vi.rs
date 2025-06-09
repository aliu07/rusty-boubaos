use crate::{ENV, errors::BadCommandError};
use std::process::Command;

pub fn vi(file_name: &str) -> anyhow::Result<()> {
    let mut path = ENV.get_current_path();
    path.push(file_name);

    Command::new("vim")
        .arg(path)
        .status()
        .map_err(|_| BadCommandError::VimError)?;

    Ok(())
}
