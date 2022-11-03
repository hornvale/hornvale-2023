use std::str::FromStr;

pub mod error;
use error::Error;

/// The `Direction` enum.
#[derive(Clone, Copy, Debug, Display, Eq, Hash, PartialEq)]
pub enum Direction {
  North,
  Northeast,
  East,
  Southeast,
  South,
  Southwest,
  West,
  Northwest,
  Up,
  Down,
  Inside,
  Outside,
}

impl FromStr for Direction {
  type Err = Error;

  fn from_str(string: &str) -> Result<Self, Self::Err> {
    use Direction::*;
    match string {
      "north" | "n" => Ok(North),
      "south" | "s" => Ok(South),
      "east" | "e" => Ok(East),
      "west" | "w" => Ok(West),
      "northeast" | "north east" | "ne" => Ok(Northeast),
      "southeast" | "south east" | "se" => Ok(Southeast),
      "northwest" | "north west" | "nw" => Ok(Northwest),
      "southwest" | "south west" | "sw" => Ok(Southwest),
      "up" | "u" => Ok(Up),
      "down" | "d" => Ok(Down),
      "inside" | "in" | "into" | "enter" => Ok(Inside),
      "outside" | "out" | "exit" => Ok(Outside),
      unknown => Err(Error::UnknownDirection(unknown.to_string())),
    }
  }
}
