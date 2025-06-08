use crate::errors::{self, BadCommandError};
use crate::memory::VARIABLE_TABLE;

pub fn set(args: Vec<&str>, args_count: usize) -> Result<(), BadCommandError> {
    if args_count < 3 {
        return Err(errors::missing_args());
    }

    if args_count > 3 {
        return Err(errors::too_many_tokens());
    }

    let variable = args[1];
    let value = args[2];

    VARIABLE_TABLE
        .set_value(variable, value)
        .unwrap_or_else(|err| {
            println!("{}", err);
        });

    Ok(())
}
