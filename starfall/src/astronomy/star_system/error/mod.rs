use crate::astronomy::star::error::Error as StarError;
use crate::astronomy::star_subsystem::error::Error as StarSubsystemError;

/// Star system errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Star Error.
  #[error("an error occurred in the star ({0})")]
  StarError(#[from] StarError),
  /// StarSubsystem Error.
  #[error("an error occurred in the star subsystem ({0})")]
  StarSubsystemError(#[from] StarSubsystemError),
  /// No suitable StarSubsystems found.
  #[error("no suitable subsystems could be generated")]
  NoSuitableSubsystemsCouldBeGenerated,
}
