// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
        _ => info(message)
    }
}
pub fn info(message: &str) -> String {
    format_log("INFO", message)
}
pub fn warn(message: &str) -> String {
    format_log("WARNING", message)
}
pub fn error(message: &str) -> String {
    format_log("ERROR", message)
}

fn debug(message: &str) -> String {
    format_log("DEBUG", message)
}

fn format_log(type_str: &str, message: &str) -> String {
    String::from(format!("[{}]: {}", type_str, message))
}
