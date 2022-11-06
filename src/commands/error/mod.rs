use crate::action_system::error::Error as ActionError;

/// Errors encountered in commands.
#[derive(Debug, Error)]
pub enum Error {
  /// User exited voluntarily.
  #[error("goodbye!")]
  UserExitError,
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
  /// An error occurred performing an action.
  #[error("an error occurred performing an action ({0})")]
  ActionError(#[from] ActionError),
}
