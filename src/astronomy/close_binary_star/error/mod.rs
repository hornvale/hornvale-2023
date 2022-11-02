use crate::astronomy::star::error::Error as StarError;

/// Close binary star-related errors.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Error, Hash, PartialEq, Serialize)]
pub enum Error {
  /// Star Error.
  #[error("an error occurred in the star ({0})")]
  StarError(#[from] StarError),
  /// Lower than MINIMUM_SEPARATION.
  #[error("the stars are too close together to be stable")]
  BinaryStarsTooCloseForComfort,
  /// The habitable zone is contained within the forbidden zone.
  #[error("the stars' habitable zone is contained within their forbidden zone")]
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  #[error("the stars' habitable zone is too close to the host stars")]
  HabitableZoneContainedWithinDangerZone,
  /// No habitable conditions found anywhere in StarSubsystem.
  #[error("the stars do not have a habitable zone")]
  NoHabitableZoneFound,
  /// An unknown error occurred.
  #[error("an unknown error occurred")]
  UnknownError,
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_errors() {
    init();

    let mut error = Error::BinaryStarsTooCloseForComfort;
    print_var!(error);
    error = Error::HabitableZoneContainedWithinForbiddenZone;
    print_var!(error);
    error = Error::HabitableZoneContainedWithinDangerZone;
    print_var!(error);
    error = Error::NoHabitableZoneFound;
    print_var!(error);
    error = Error::UnknownError;
    print_var!(error);
    error = Error::StarError(StarError::UnknownError);
    print_var!(error);
    error = StarError::UnknownError.into();
    print_var!(error);
  }
}
