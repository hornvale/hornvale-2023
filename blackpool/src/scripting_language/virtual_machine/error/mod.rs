use crate::scripting_language::compiler::error::Error as CompilerError;

pub mod compilation;
use compilation::CompilationError;
pub mod runtime;
use runtime::RuntimeError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
  /// A compilation error occurred.
  #[error("a compilation error occurred ({0})")]
  CompilationError(#[from] CompilationError),
  /// A compiler error occurred.
  #[error("a compiler error occurred ({0})")]
  CompilerError(#[from] CompilerError),
  /// A runtime error occurred.
  #[error("a runtime error occurred ({0})")]
  RuntimeError(#[from] RuntimeError),
}
