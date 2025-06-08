use super::commands;
use crate::errors::BadCommandError;

const MAX_ARGS_COUNT: usize = 7;

pub fn interpret(args: Vec<&str>, args_count: usize) -> Result<i32, BadCommandError> {
    if args_count < 1 {
        return Err(BadCommandError::MissingArgs);
    }

    if args_count > MAX_ARGS_COUNT {
        return Err(BadCommandError::TooManyTokens);
    }

    if args[0] == "quit" && args_count == 1 {
        commands::quit();
    };

    match args[0] {
        "help" => {
            if args_count > 1 {
                return Err(BadCommandError::TooManyTokens);
            }

            commands::help()
        }
        "set" => {
            if args_count < 3 {
                return Err(BadCommandError::MissingArgs);
            }

            if args_count > 3 {
                return Err(BadCommandError::TooManyTokens);
            }

            let variable = args[1];
            let value = args[2];
            commands::set(variable, value)?
        }
        "remove" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs);
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens);
            }

            let variable = args[1];
            commands::remove(variable)?
        }
        "print" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs);
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens);
            }

            let variable = args[1];
            commands::print(variable)?
        }
        "echo" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs);
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens);
            }

            let variable = args[1];
            commands::echo(variable)?;
        }
        "pwd" => {
            if args_count > 1 {
                return Err(BadCommandError::TooManyTokens);
            }

            commands::pwd()?;
        }
        _ => return Err(BadCommandError::UnknownCommand),
    }

    // Success
    Ok(0)
}
