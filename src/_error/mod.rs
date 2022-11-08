use anyhow::Error as AnyError;

/// The crate-level `Error` type.
///
/// It's useful to have very specific error messages, but we also run into
/// combinatorial scaling issues if we need to convert from e.g. an Action
/// Error into a Command Error and so forth.
///
/// I think the best approach is probably to use the extremely specific
/// error types, as constructed by `thiserror`, etc, within individual
/// systems, but convert them at the system boundaries into `anyhow` errors.
///
/// This type is intended to accomplish that.
#[derive(Debug, Error)]
pub enum Error {
  /// Any error occurred.
  #[error(transparent)]
  AnyError(#[from] AnyError),
}
