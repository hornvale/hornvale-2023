use crate::scripting_language::compiler::error::Error as CompilerError;

pub mod runtime;
use runtime::RuntimeError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
  /// A compiler error occurred.
  #[error("a compiler error occurred ({0})")]
  CompilerError(#[from] CompilerError),
  /// A runtime error occurred.
  #[error("a runtime error occurred ({0})")]
  RuntimeError(#[from] RuntimeError),
}
