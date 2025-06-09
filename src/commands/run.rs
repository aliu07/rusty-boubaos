use crate::{errors::BadCommandError, parse_input};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run(path: &str) -> anyhow::Result<()> {
    // Check file extension
    let file_path = Path::new(path);

    if file_path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
        return Err(BadCommandError::InvalidFileFormat.into());
    }

    // Read contents
    let file = File::open(path).map_err(|_| BadCommandError::FileNotFound(String::from(path)))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|_| BadCommandError::FileReadError)?;

        parse_input(&line)?;
    }

    Ok(())
}
