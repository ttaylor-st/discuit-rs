//! This module contains things used internally by `discuit-rs`.

use serde::{Deserialize, Serialize};
use std::fmt;

/// LogLevel represents the level of logging to use.
/// The levels are, in order of verbosity:
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum LogLevel {
    /// Log everything.
    /// This is the most verbose level and should be used for debugging.
    /// This level includes all other levels as well as request and response bodies.
    Debug,
    /// Log informational messages.
    /// This level includes less verbose levels as well as information about the client
    /// and requests that are made, e.g., the URL of the request.
    Info,
    /// Log warnings.
    /// This level includes less verbose levels as well as warnings about potential issues.
    /// This level should be used for non-critical issues.
    /// The client will continue to work, but the issue should be addressed.
    Warning,
    /// Log errors.
    /// This level includes less verbose levels as well as errors that occur during the client's operation.
    /// This level should be used for critical issues that prevent the client from working.
    /// The client may not work as expected, and the issue should be addressed.
    Error,
    /// Log nothing.
    /// This level should be used to silence all logging. No messages will be printed.
    Silent,
}

impl Default for LogLevel {
    fn default() -> Self {
        LogLevel::Info
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Silent => write!(f, "SILENT"),
        }
    }
}
