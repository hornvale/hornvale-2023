use crate::astronomy::stellar_neighborhood::error::Error as StellarNeighborhoodError;

/// Galaxy-class errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Stellar Neighborhood Error.
  StellarNeighborhoodError(StellarNeighborhoodError),
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    StellarNeighborhoodError(stellar_neighborhood_error) => format!(
      "an error occurred in the stellar neighborhood ({})",
      honeyholt_brief!(stellar_neighborhood_error)
    ),
  }
});

impl From<StellarNeighborhoodError> for Error {
  #[named]
  fn from(error: StellarNeighborhoodError) -> Self {
    Error::StellarNeighborhoodError(error)
  }
}
