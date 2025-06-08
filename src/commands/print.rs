use crate::errors::{self, BadCommandError};
use crate::memory::VARIABLE_TABLE;

pub fn print(args: Vec<&str>, args_count: usize) -> Result<(), BadCommandError> {
    if args_count < 2 {
        return Err(errors::missing_args());
    }

    if args_count > 2 {
        return Err(errors::too_many_tokens());
    }

    let variable = args[1];
    let value = VARIABLE_TABLE.get_value(variable)?;

    println!("{}", value);

    Ok(())
}
