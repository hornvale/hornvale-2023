/// Errors encountered attempting to perform A* pathfinding.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Failed to create a plan.
  #[error("failed to create a plan")]
  FailedToCreateAPlan,
  /// Not found.
  #[error("not found")]
  NotFound,
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}
