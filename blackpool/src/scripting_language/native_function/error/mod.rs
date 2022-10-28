/// Errors encountered during the parsing process.
#[derive(Clone, Debug, Error)]
pub enum Error {
  /// Unknown error.
  #[error("an unknown error occurred")]
  UnknownError,
  /// Execution error.
  #[error("an error occurred executing the native function ({0})")]
  ExecutionError(String),
}
