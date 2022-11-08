use specs::prelude::*;
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::map::Direction;
use crate::map::Passage;

/// The `HasPassages` component.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct HasPassages {
  pub north: Option<Passage>,
  pub northeast: Option<Passage>,
  pub east: Option<Passage>,
  pub southeast: Option<Passage>,
  pub south: Option<Passage>,
  pub southwest: Option<Passage>,
  pub west: Option<Passage>,
  pub northwest: Option<Passage>,
  pub up: Option<Passage>,
  pub down: Option<Passage>,
  pub inside: Option<Passage>,
  pub outside: Option<Passage>,
}

impl HasPassages {
  pub fn get_values(&self) -> Vec<&Option<Passage>> {
    vec![
      &self.north,
      &self.northeast,
      &self.east,
      &self.southeast,
      &self.south,
      &self.southwest,
      &self.west,
      &self.northwest,
      &self.up,
      &self.down,
      &self.inside,
      &self.outside,
    ]
  }

  pub fn get_value(&self, direction: &Direction) -> &Option<Passage> {
    use Direction::*;
    match direction {
      North => &self.north,
      Northeast => &self.northeast,
      East => &self.east,
      Southeast => &self.southeast,
      South => &self.south,
      Southwest => &self.southwest,
      West => &self.west,
      Northwest => &self.northwest,
      Up => &self.up,
      Down => &self.down,
      Inside => &self.inside,
      Outside => &self.outside,
    }
  }

  pub fn set_passage(&mut self, direction: &Direction, passage: Option<Passage>) {
    use Direction::*;
    match direction {
      Northwest => self.northwest = passage,
      North => self.north = passage,
      Northeast => self.northeast = passage,
      East => self.east = passage,
      Southeast => self.southeast = passage,
      South => self.south = passage,
      Southwest => self.southwest = passage,
      West => self.west = passage,
      Up => self.up = passage,
      Down => self.down = passage,
      Inside => self.inside = passage,
      Outside => self.outside = passage,
    }
  }

  pub fn get_passage_to(&self, direction: &Direction) -> &Option<Passage> {
    self.get_value(direction)
  }

  pub fn has_passage_to(&self, direction: &Direction) -> bool {
    self.get_value(direction).is_some()
  }

  pub fn get_passages(&self) -> Vec<&Passage> {
    self.get_values().into_iter().filter_map(Option::as_ref).collect()
  }

  pub fn get_directions(&self) -> Vec<Direction> {
    self
      .get_passages()
      .iter()
      .map(|exit| exit.direction)
      .collect::<Vec<Direction>>()
  }
}

impl Display for HasPassages {
  fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
    let mut directions = self.get_directions();
    let string = match directions.len() {
      0 => "There are no visible passages.".into(),
      1 => format!("There is a visible passages to the {}.", directions.pop().unwrap()),
      2 => format!(
        "There are visible passages to the {} and {}.",
        directions.pop().unwrap(),
        directions.pop().unwrap()
      ),
      _ => {
        let last = directions.pop().unwrap();
        let others = directions
          .iter()
          .map(|d| d.get_lowercase())
          .collect::<Vec<&'static str>>()
          .join(", ");
        format!("There are visible passages to the {}, and {}.", others, last)
      },
    };
    write!(formatter, "{}", string)
  }
}
