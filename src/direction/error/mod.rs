/// Errors encountered dealing with directions.
#[derive(Clone, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Unknown direction.
  #[error("unknown direction ({0})")]
  UnknownDirection(String),
}
