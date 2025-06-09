use crate::environment::ENV;
use crate::errors::BadCommandError;
use std::{env, fs, path::Path};

pub fn cd(path: &str) -> anyhow::Result<()> {
    let mut target_path = ENV.get_current_path();
    target_path.push(Path::new(path));
    let target_path = fs::canonicalize(target_path)
        .map_err(|_| BadCommandError::PathDoesNotExist(String::from(path)))?;

    // Prevent exiting from root directory
    if target_path.starts_with(ENV.get_root_path()) {
        ENV.set_current_path(&target_path);
        env::set_current_dir(&target_path).map_err(move |_| {
            BadCommandError::PathDoesNotExist(target_path.display().to_string())
        })?;
    } else {
        return Err(BadCommandError::ExitRootDirectoryError.into());
    }

    Ok(())
}
