use crate::errors::BadCommandError;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run(path: &str) -> Result<(), BadCommandError> {
    let file = File::open(path).map_err(|_| BadCommandError::FileNotFound(String::from(path)))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.map_err(|_| BadCommandError::FileReadError)?;

        println!("{}", line);
    }

    Ok(())
}
