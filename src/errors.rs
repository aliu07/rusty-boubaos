#[derive(Debug)]
pub enum BadCommandError {
    UnknownCommand,
    MissingArgs,
    TooManyTokens,
    VariableTableError(String),
    VariableDoesNotExist(String),
    VariableTableFull,
    FileNotFound(String),
    CurrentDirectoryReadError,
    DirectoryEntryReadError,
    FileReadError,
    InvalidFileFormat,
    DirectoryAlreadyExists(String),
    PathDoesNotExist(String),
    CreateFileError,
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
            BadCommandError::CurrentDirectoryReadError => {
                write!(f, "Bad command: Error reading current directory")
            }
            BadCommandError::FileReadError => write!(f, "Bad command: Error reading file contents"),
            BadCommandError::InvalidFileFormat => write!(f, "Bad commad: Invalid file format"),
            BadCommandError::DirectoryAlreadyExists(dirname) => {
                write!(f, "Bad commad: Directory '{}' already exists", dirname)
            }
            BadCommandError::DirectoryEntryReadError => {
                write!(f, "Bad command: Error reading directory entry")
            }
            BadCommandError::PathDoesNotExist(path) => {
                write!(f, "Bad command: The path '{}' does not exist", path)
            }
            BadCommandError::CreateFileError => {
                write!(f, "Bad command: Error creating file")
            }
        }
    }
}

impl std::error::Error for BadCommandError {}
