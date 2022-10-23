pub mod compilation;
use compilation::CompilationError;
pub mod runtime;
use runtime::RuntimeError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// A compilation error occurred.
  #[error("a compilation error occurred ({0})")]
  CompilationError(#[from] CompilationError),
  /// A runtime error occurred.
  #[error("a runtime error occurred ({0})")]
  RuntimeError(#[from] RuntimeError),
}
