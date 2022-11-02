/// Errors encountered in parsing.
#[derive(Debug, Error)]
pub enum Error {
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}
