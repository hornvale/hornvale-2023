/// Errors encountered in compiling or executing a script.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}
