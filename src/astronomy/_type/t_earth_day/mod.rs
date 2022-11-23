use super::TEarthHour;
use super::TEarthYear;
use crate::astronomy::_constants::*;

/// The `TEarthDay` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct TEarthDay(pub f64);

impl From<TEarthHour> for TEarthDay {
  fn from(original: TEarthHour) -> Self {
    Self(original.0 / EARTH_HOURS_PER_DAY.0)
  }
}

impl From<TEarthYear> for TEarthDay {
  fn from(original: TEarthYear) -> Self {
    Self(original.0 * EARTH_DAYS_PER_YEAR.0)
  }
}
