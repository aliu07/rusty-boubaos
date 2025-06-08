use crate::errors::BadCommandError;
use crate::memory::VARIABLE_TABLE;

pub fn set(variable: &str, value: &str) -> Result<(), BadCommandError> {
    VARIABLE_TABLE.set_value(variable, value)?;

    Ok(())
}
