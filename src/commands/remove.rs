use crate::{errors::BadCommandError, memory::VARIABLE_TABLE};

pub fn remove(variable: &str) -> Result<(), BadCommandError> {
    VARIABLE_TABLE.remove_value(variable)?;

    Ok(())
}
