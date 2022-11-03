/// Errors encountered in parsing.
#[derive(Debug, Error)]
pub enum Error {
  /// User exited voluntarily.
  #[error("goodbye!")]
  UserExitError,
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}
