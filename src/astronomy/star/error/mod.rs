/// Star-related errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
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
