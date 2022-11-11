use rand::distributions::Standard;
use rand::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

pub mod error;
use error::Error;

/// The `Direction` enum.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

impl Direction {
  pub fn get_inverse(&self) -> Direction {
    use Direction::*;
    match self {
      Northwest => Southeast,
      North => South,
      Northeast => Southwest,
      East => West,
      West => East,
      Southeast => Northwest,
      South => North,
      Southwest => Northeast,
      Up => Down,
      Down => Up,
      Inside => Outside,
      Outside => Inside,
    }
  }

  pub fn get_name(&self) -> &'static str {
    use Direction::*;
    match self {
      Northwest => "Northwest",
      North => "North",
      Northeast => "Northeast",
      East => "East",
      West => "West",
      Southeast => "Southeast",
      South => "South",
      Southwest => "Southwest",
      Up => "Up",
      Down => "Down",
      Inside => "Inside",
      Outside => "Outside",
    }
  }

  pub fn get_short_name(&self) -> &'static str {
    use Direction::*;
    match self {
      Northwest => "nw",
      North => "n",
      Northeast => "ne",
      East => "e",
      West => "w",
      Southeast => "se",
      South => "s",
      Southwest => "sw",
      Up => "up",
      Down => "down",
      Inside => "in",
      Outside => "out",
    }
  }

  pub fn get_lowercase(&self) -> &'static str {
    use Direction::*;
    match self {
      Northwest => "northwest",
      North => "north",
      Northeast => "northeast",
      East => "east",
      West => "west",
      Southeast => "southeast",
      South => "south",
      Southwest => "southwest",
      Up => "up",
      Down => "down",
      Inside => "inside",
      Outside => "outside",
    }
  }

  pub fn is_compass_direction(&self) -> bool {
    use Direction::*;
    !matches!(self, Up | Down | Inside | Outside)
  }
}

impl Display for Direction {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    write!(formatter, "{}", self.get_lowercase())
  }
}

impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    let index: u8 = rng.gen_range(0..12);
    use Direction::*;
    match index {
      0 => North,
      1 => Northeast,
      2 => East,
      3 => Southeast,
      4 => South,
      5 => Southwest,
      6 => West,
      7 => Northwest,
      8 => Up,
      9 => Down,
      10 => Inside,
      11 => Outside,
      _ => unreachable!(),
    }
  }
}
