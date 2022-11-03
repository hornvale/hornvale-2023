use std::str::FromStr;

use crate::commands::command::Command;
use crate::direction::Direction;
use crate::entity::Entity;
use crate::parsing::error::Error;
use crate::parsing::parser::Parser;
use crate::player::Player;

/// The `TwoWord` type.
///
/// This will actually be slightly more than a two-word parser, but who cares?
#[derive(Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq)]
pub struct TwoWord {}

impl Parser for TwoWord {
  /// Parse two (or more) words of input.

  fn parse_input(&mut self, input: &str) -> Result<Option<String>, Error> {
    let player = Player::new(Entity {});
    let words = input.split(' ').map(str::to_string).collect::<Vec<String>>();
    let word0 = words.get(0).cloned().unwrap_or_else(|| "".to_string());
    let word1 = words.get(1).cloned().unwrap_or_else(|| "".to_string());
    let result = match (word0.as_str(), word1.as_str()) {
      ("look", direction) if Direction::from_str(direction).is_ok() => {
        Command::LookDirection(player, Direction::from_str(direction)?).execute()?
      },
      ("look", _) => Command::Look(player).execute()?,
      ("go", direction) | (direction, _) if Direction::from_str(direction).is_ok() => {
        Command::GoDirection(player, Direction::from_str(direction)?).execute()?
      },
      ("quit", _) => Command::Quit(player).execute()?,
      (_, _) => todo!(),
    };
    Ok(result)
  }
}
