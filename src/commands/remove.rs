use crate::{
    errors::{self, BadCommandError},
    memory::VARIABLE_TABLE,
};

pub fn remove(args: Vec<&str>, args_count: usize) -> Result<(), BadCommandError> {
    if args_count < 2 {
        return Err(errors::missing_args());
    }

    if args_count > 2 {
        return Err(errors::too_many_tokens());
    }

    let variable = args[1];
    VARIABLE_TABLE.remove_value(variable)?;

    Ok(())
}
