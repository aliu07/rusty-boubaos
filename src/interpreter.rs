use super::commands;
use crate::errors::BadCommandError;

const MAX_ARGS_COUNT: usize = 7;

pub fn interpret(args: Vec<&str>, args_count: usize) -> anyhow::Result<i32> {
    if args_count < 1 {
        return Err(BadCommandError::MissingArgs.into());
    }

    if args_count > MAX_ARGS_COUNT {
        return Err(BadCommandError::TooManyTokens.into());
    }

    if args[0] == "quit" && args_count == 1 {
        commands::quit();
    };

    match args[0] {
        "help" => {
            if args_count > 1 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            commands::help()
        }
        "set" => {
            if args_count < 3 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 3 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let variable = args[1];
            let value = args[2];
            commands::set(variable, value)?
        }
        "remove" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let variable = args[1];
            commands::remove(variable)?
        }
        "print" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let variable = args[1];
            commands::print(variable)?
        }
        "echo" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let variable = args[1];
            commands::echo(variable)?;
        }
        "pwd" => {
            if args_count > 1 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            commands::pwd()?;
        }
        "run" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let path = args[1];
            commands::run(path)?;
        }
        "mkdir" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let dirname = args[1];
            commands::mkdir(dirname)?;
        }
        "ls" => {
            if args_count > 1 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            commands::ls()?;
        }
        "cd" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let target = args[1];
            commands::cd(target)?;
        }
        "touch" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let file_name = args[1];
            commands::touch(file_name)?;
        }
        "rm" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let file_name = args[1];
            commands::rm(file_name)?;
        }
        "rmdir" => {
            if args_count < 2 {
                return Err(BadCommandError::MissingArgs.into());
            }

            if args_count > 2 {
                return Err(BadCommandError::TooManyTokens.into());
            }

            let dir_name = args[1];
            commands::rmdir(dir_name)?;
        }
        _ => return Err(BadCommandError::UnknownCommand.into()),
    }

    // Success
    Ok(0)
}
