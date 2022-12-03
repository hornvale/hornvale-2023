use super::TEarthDay;
use super::TEarthYear;
use crate::astronomy::_constant::*;

/// The `TEarthHour` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct TEarthHour(pub f64);

impl From<TEarthDay> for TEarthHour {
  fn from(original: TEarthDay) -> Self {
    Self(original.0 * EARTH_HOURS_PER_DAY.0)
  }
}

impl From<TEarthYear> for TEarthHour {
  fn from(original: TEarthYear) -> Self {
    Self(original.0 * EARTH_HOURS_PER_DAY.0 * EARTH_DAYS_PER_YEAR.0)
  }
}
