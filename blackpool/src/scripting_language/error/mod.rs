use crate::scripting_language::virtual_machine::error::Error as GeneralError;

/// Errors encountered in compiling or executing a script.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
  /// A general error occurred.
  #[error("an error occurred ({0})")]
  GeneralError(#[from] Box<GeneralError>),
}
