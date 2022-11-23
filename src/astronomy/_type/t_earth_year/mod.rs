use super::TEarthDay;
use super::TEarthHour;
use crate::astronomy::_constants::*;

/// The `TEarthYear` newtype.
#[derive(Add, Clone, Copy, Debug, Default, Deserialize, Display, Div, Mul, PartialEq, PartialOrd, Serialize, Sub)]
pub struct TEarthYear(pub f64);

impl From<TEarthHour> for TEarthYear {
  fn from(original: TEarthHour) -> Self {
    Self(original.0 / EARTH_HOURS_PER_DAY.0 / EARTH_DAYS_PER_YEAR.0)
  }
}

impl From<TEarthDay> for TEarthYear {
  fn from(original: TEarthDay) -> Self {
    Self(original.0 / EARTH_DAYS_PER_YEAR.0)
  }
}
