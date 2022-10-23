/// Errors encountered at compilation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum CompilationError {
  /// Unknown error.
  #[error("unknown error")]
  UnknownError,
}
