use crate::astronomy::star::error::Error as StarError;

/// Close binary star-related errors.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
  /// Star Error.
  StarError(StarError),
  /// Lower than MINIMUM_SEPARATION.
  BinaryStarsTooCloseForComfort,
  /// The habitable zone is contained within the forbidden zone.
  HabitableZoneContainedWithinForbiddenZone,
  /// The habitable zone isn't sufficiently far from the host stars.
  HabitableZoneContainedWithinDangerZone,
  /// No habitable conditions found anywhere in StarSubsystem.
  NoHabitableZoneFound,
}

honeyholt_define_brief!(Error, |error: &Error| {
  use Error::*;
  match error {
    BinaryStarsTooCloseForComfort => "the stars are too close together to be stable".to_string(),
    HabitableZoneContainedWithinForbiddenZone => {
      "the stars' habitable zone is contained within their forbidden zone".to_string()
    },
    HabitableZoneContainedWithinDangerZone => "the stars' habitable zone is too close to the host stars".to_string(),
    NoHabitableZoneFound => "the stars do not have a habitable zone".to_string(),
    StarError(star_error) => format!("an error occurred in the star ({})", honeyholt_brief!(star_error)),
  }
});

impl From<StarError> for Error {
  #[named]
  fn from(error: StarError) -> Self {
    Error::StarError(error)
  }
}
