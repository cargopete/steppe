//! Error types for Steppe
//!
//! Uses `miette` for pretty error reporting with source spans and help text.

use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

/// Main error type for Steppe operations
#[derive(Error, Diagnostic, Debug)]
pub enum SteppeError {
    #[error("Configuration file not found")]
    #[diagnostic(
        code(steppe::config::not_found),
        help("Create a Steppe.toml in your project root, or specify one with --config")
    )]
    ConfigNotFound {
        searched: Vec<PathBuf>,
    },

    #[error("Failed to parse configuration")]
    #[diagnostic(code(steppe::config::parse))]
    ConfigParse {
        #[source]
        source: toml::de::Error,
        path: PathBuf,
    },

    #[error("Task '{name}' not found")]
    #[diagnostic(
        code(steppe::task::not_found),
        help("Run `steppe list` to see available tasks")
    )]
    TaskNotFound {
        name: String,
        available: Vec<String>,
    },

    #[error("Circular dependency detected: {cycle}")]
    #[diagnostic(
        code(steppe::task::cycle),
        help("Check the 'depends' field in your task definitions")
    )]
    CyclicDependency {
        cycle: String,
    },

    #[error("Task '{task}' failed with exit code {code}")]
    #[diagnostic(code(steppe::exec::failed))]
    TaskFailed {
        task: String,
        code: i32,
        #[help]
        stderr: Option<String>,
    },

    #[error("Command not found: {command}")]
    #[diagnostic(
        code(steppe::exec::command_not_found),
        help("Ensure the command is installed and in your PATH")
    )]
    CommandNotFound {
        command: String,
    },

    #[error("Script execution failed in task '{task}'")]
    #[diagnostic(code(steppe::script::failed))]
    ScriptFailed {
        task: String,
        #[source]
        source: Box<rhai::EvalAltResult>,
    },

    #[error("Invalid task configuration")]
    #[diagnostic(code(steppe::config::invalid_task))]
    InvalidTask {
        task: String,
        reason: String,
    },

    #[error("I/O error")]
    #[diagnostic(code(steppe::io))]
    Io(#[from] std::io::Error),

    #[error("Cache error: {message}")]
    #[diagnostic(code(steppe::cache))]
    Cache {
        message: String,
    },

    #[error("Watch error")]
    #[diagnostic(code(steppe::watch))]
    Watch {
        #[source]
        source: notify::Error,
    },
}

/// Result type alias for Steppe operations
pub type Result<T> = std::result::Result<T, SteppeError>;
