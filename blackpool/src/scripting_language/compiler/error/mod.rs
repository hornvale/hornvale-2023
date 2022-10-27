use crate::scripting_language::token::Token;

/// Errors encountered during the parsing process.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum Error {
  /// Unknown error.
  #[error("an unknown error occurred")]
  UnknownError,
  /// Attempted to read the variable in its own initializer.
  #[error("attempted to read variable in its own initializer {0:#?}")]
  AttemptedToReadVariableInOwnInitializer(Option<Token>),
}
