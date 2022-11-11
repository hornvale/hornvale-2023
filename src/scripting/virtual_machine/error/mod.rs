use crate::scripting::interpreter::error::Error as InterpreterError;
use crate::scripting::native_function::error::Error as NativeFunctionError;

pub mod runtime;
use runtime::RuntimeError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Error)]
pub enum Error {
  /// An interpreter error occurred.
  #[error("an interpreter error occurred ({0})")]
  InterpreterError(#[from] InterpreterError),
  /// A runtime error occurred.
  #[error("a runtime error occurred ({0})")]
  RuntimeError(#[from] RuntimeError),
  /// A native function error occurred.
  #[error("an error occurred in a native function ({0})")]
  NativeFunctionError(#[from] NativeFunctionError),
}
