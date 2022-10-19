/// Star-related errors.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum Error {
  /// Lower than MAIN_SEQUENCE_STAR_MASS_LOWER_BOUND.
  #[error("its mass is too low to be a main-sequence star")]
  MassTooLowForMainSequence,
  /// Higher than MAIN_SEQUENCE_STAR_MASS_UPPER_BOUND.
  #[error("its mass is too high to be a main-sequence star")]
  MassTooHighForMainSequence,
  /// Lower than MINIMUM_STAR_AGE_TO_SUPPORT_LIFE.
  #[error("it is too young to support life")]
  TooYoungToSupportLife,
  /// Lower than MINIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  #[error("its mass is too low to support life")]
  MassTooLowToSupportLife,
  /// Higher than MAXIMUM_STAR_MASS_TO_SUPPORT_LIFE.
  #[error("its mass is too high to support life")]
  MassTooHighToSupportLife,
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    MassTooLowForMainSequence => "its mass is too low to be a main-sequence star".to_string(),
    MassTooHighForMainSequence => "its mass is too high to be a main-sequence star".to_string(),
    TooYoungToSupportLife => "it is too young to support life".to_string(),
    MassTooLowToSupportLife => "its mass is too low to support life".to_string(),
    MassTooHighToSupportLife => "its mass is too high to support life".to_string(),
    UnknownError => "an unknown error occurred".to_string(),
  }
});
