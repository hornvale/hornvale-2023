use std::str::FromStr;

use crate::direction::Direction;
use crate::parsing::error::Error;

/// The `Type` enum.
///
/// These are types of tokens.
pub enum Type {
  Direction(Direction),
  Other(String),
}

impl FromStr for Type {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    use Type::*;
    match string {
      string if Direction::from_str(string).is_ok()
      other => Ok(Other(other)),
    }
  }
}
