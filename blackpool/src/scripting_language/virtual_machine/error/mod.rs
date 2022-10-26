use crate::scripting_language::interpreter::error::Error as InterpreterError;

pub mod runtime;
use runtime::RuntimeError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
  /// An interpreter error occurred.
  #[error("an interpreter error occurred ({0})")]
  InterpreterError(#[from] InterpreterError),
  /// A runtime error occurred.
  #[error("a runtime error occurred ({0})")]
  RuntimeError(#[from] RuntimeError),
}
