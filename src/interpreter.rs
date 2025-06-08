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

    if args[0] == "quit" && args_count == 1 {
        commands::quit();
    };

    match args[0] {
        "help" => {
            if args_count > 1 {
                return Err(errors::too_many_tokens());
            }

            commands::help()?
        }
        "set" => {
            if args_count < 3 {
                return Err(errors::missing_args());
            }

            if args_count > 3 {
                return Err(errors::too_many_tokens());
            }

            let variable = args[1];
            let value = args[2];
            commands::set(variable, value)?
        }
        "remove" => {
            if args_count < 2 {
                return Err(errors::missing_args());
            }

            if args_count > 2 {
                return Err(errors::too_many_tokens());
            }

            let variable = args[1];
            commands::remove(variable)?
        }
        "print" => {
            if args_count < 2 {
                return Err(errors::missing_args());
            }

            if args_count > 2 {
                return Err(errors::too_many_tokens());
            }

            let variable = args[1];
            commands::print(variable)?
        }
        _ => Err(errors::unknown_command())?,
    }

    // Success
    Ok(0)
}
