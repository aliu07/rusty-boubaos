#[derive(Debug)]
pub enum BadCommandError {
    UnknownCommand,
    MissingArgs,
    TooManyTokens,
    VariableTableError(String),
    VariableDoesNotExist(String),
    VariableTableFull,
}

impl std::fmt::Display for BadCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BadCommandError::UnknownCommand => write!(f, "Bad command: Unknown command"),
            BadCommandError::MissingArgs => write!(f, "Bad command: Missing arguments"),
            BadCommandError::TooManyTokens => write!(f, "Bad command: Too many tokens"),
            BadCommandError::VariableTableError(err) => write!(f, "Variable table error: {}", err),
            BadCommandError::VariableDoesNotExist(var) => {
                write!(f, "Bad command: Variable '{}' does not exist in table", var)
            }
            BadCommandError::VariableTableFull => write!(f, "Bad command: Variable table is full"),
        }
    }
}

impl std::error::Error for BadCommandError {}

pub fn unknown_command() -> BadCommandError {
    BadCommandError::UnknownCommand
}

pub fn missing_args() -> BadCommandError {
    BadCommandError::MissingArgs
}

pub fn too_many_tokens() -> BadCommandError {
    BadCommandError::TooManyTokens
}

pub fn variable_table_full() -> BadCommandError {
    BadCommandError::VariableTableFull
}
