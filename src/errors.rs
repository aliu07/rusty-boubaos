#[derive(Debug)]
pub enum BadCommandError {
    UnknownCommand,
    MissingArgs,
    TooManyTokens,
    VariableTableError(String),
    VariableDoesNotExist(String),
    VariableTableFull,
    FileNotFound(String),
    PwdError,
    FileReadError,
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
            BadCommandError::FileNotFound(path) => {
                write!(f, "Bad command: Error reading file at '{}'", path)
            }
            BadCommandError::PwdError => write!(f, "Bad command: Could not execute pwd"),
            BadCommandError::FileReadError => write!(f, "Bad command: Error reading file contents"),
        }
    }
}

impl std::error::Error for BadCommandError {}
