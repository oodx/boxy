// Theme error handling for library embedding
// Replaces process::exit with proper Result types

use std::fmt;

/// Theme subsystem errors
/// Some variants are for external library consumers
#[derive(Debug)]
#[allow(dead_code)]
pub enum ThemeError {
    /// Invalid theme name
    InvalidName(String),
    /// Theme not found
    NotFound(String),
    /// Theme already exists
    AlreadyExists(String),
    /// File I/O error
    Io(std::io::Error),
    /// YAML parsing error
    Yaml(String),
    /// Engine initialization failed
    EngineInit(String),
    /// Validation failed
    Validation(String),
    /// Invalid command/action
    InvalidCommand(String),
    /// Missing argument
    MissingArgument(String),
}

impl fmt::Display for ThemeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ThemeError::InvalidName(msg) => write!(f, "Invalid theme name: {}", msg),
            ThemeError::NotFound(name) => write!(f, "Theme '{}' not found", name),
            ThemeError::AlreadyExists(name) => write!(f, "Theme '{}' already exists", name),
            ThemeError::Io(err) => write!(f, "I/O error: {}", err),
            ThemeError::Yaml(msg) => write!(f, "YAML error: {}", msg),
            ThemeError::EngineInit(msg) => write!(f, "Engine initialization failed: {}", msg),
            ThemeError::Validation(msg) => write!(f, "Validation failed: {}", msg),
            ThemeError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
            ThemeError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
        }
    }
}

impl std::error::Error for ThemeError {}

impl From<std::io::Error> for ThemeError {
    fn from(err: std::io::Error) -> Self {
        ThemeError::Io(err)
    }
}

/// Result type for theme operations
pub type ThemeResult<T> = Result<T, ThemeError>;

/// Exit code suggestions for CLI usage
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ExitCode {
    Success = 0,
    GeneralError = 1,
    InvalidArgument = 2,
    NotFound = 3,
    AlreadyExists = 4,
    IoError = 5,
    ValidationError = 6,
}

impl From<&ThemeError> for ExitCode {
    fn from(err: &ThemeError) -> Self {
        match err {
            ThemeError::InvalidName(_) | ThemeError::InvalidCommand(_) | ThemeError::MissingArgument(_) => ExitCode::InvalidArgument,
            ThemeError::NotFound(_) => ExitCode::NotFound,
            ThemeError::AlreadyExists(_) => ExitCode::AlreadyExists,
            ThemeError::Io(_) => ExitCode::IoError,
            ThemeError::Validation(_) => ExitCode::ValidationError,
            ThemeError::EngineInit(_) | ThemeError::Yaml(_) => ExitCode::GeneralError,
        }
    }
}