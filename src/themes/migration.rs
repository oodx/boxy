// Migration utilities for process::exit removal
// Provides compatibility layer for gradual refactoring

use super::error::{ThemeError, ThemeResult, ExitCode};

/// Handle theme error in CLI context
///
/// For CLI usage, this prints the error and suggests exit code.
/// For library usage, errors should be propagated as Results.
pub fn handle_cli_error(err: ThemeError) -> ! {
    eprintln!("Error: {}", err);

    // Additional context for specific errors
    match &err {
        ThemeError::AlreadyExists(name) => {
            eprintln!("Use 'boxy theme edit {}' to modify it", name);
        }
        ThemeError::NotFound(name) => {
            eprintln!("Use 'boxy theme list' to see available themes");
            eprintln!("Use 'boxy theme create {}' to create a new theme", name);
        }
        ThemeError::InvalidCommand(_) => {
            eprintln!("Use 'boxy theme help' for more information");
        }
        _ => {}
    }

    let exit_code = ExitCode::from(&err);
    std::process::exit(exit_code as i32)
}

/// Convert validation error string to ThemeError
pub fn validation_error(msg: impl Into<String>) -> ThemeError {
    ThemeError::Validation(msg.into())
}

/// Convert engine init error string to ThemeError
pub fn engine_error(msg: impl Into<String>) -> ThemeError {
    ThemeError::EngineInit(msg.into())
}

/// Wrapper for theme commands that converts Results to exit codes
///
/// This allows gradual migration of theme commands to Result types
/// while maintaining CLI compatibility.
pub fn run_theme_command<F, T>(f: F) -> T
where
    F: FnOnce() -> ThemeResult<T>,
    T: Default,
{
    match f() {
        Ok(result) => result,
        Err(err) => {
            handle_cli_error(err);
        }
    }
}

/// Check if running in library context
///
/// This can be used to determine whether to exit or return errors.
/// For now, always returns false (CLI mode) until full migration.
pub fn is_library_context() -> bool {
    // TODO: Implement detection based on environment or config
    // For now, assume CLI context for backward compatibility
    false
}