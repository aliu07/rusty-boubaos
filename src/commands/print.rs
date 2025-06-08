use crate::errors::BadCommandError;
use crate::memory::VARIABLE_TABLE;

pub fn print(variable: &str) -> Result<(), BadCommandError> {
    let value = VARIABLE_TABLE.get_value(variable)?;

    println!("{}", value);

    Ok(())
}
