use crate::scripting_language::virtual_machine::error::Error as GeneralError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// A general error occurred.
  #[error("an error occurred ({0})")]
  GeneralError(#[from] GeneralError),
}
