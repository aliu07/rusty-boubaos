use super::commands;
use crate::errors::{self, BadCommandError};

const MAX_ARGS_COUNT: usize = 7;

pub fn interpret(args: Vec<&str>, args_count: usize) -> Result<i32, BadCommandError> {
    if args_count < 1 {
        return Err(errors::missing_args());
    }

    if args_count > MAX_ARGS_COUNT {
        return Err(errors::too_many_tokens());
    }

    if args[0] == "quit" {
        commands::quit();
    }

    match args[0] {
        "help" => commands::help(args_count)?,
        "set" => commands::set(args, args_count)?,
        "print" => commands::print(args, args_count)?,
        _ => Err(errors::unknown_command())?,
    }

    // Success
    Ok(0)
}
