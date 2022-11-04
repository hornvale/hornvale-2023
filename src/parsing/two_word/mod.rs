use std::collections::HashMap;
use std::str::FromStr;

use crate::commands::command::Command;
use crate::components::is_in_room::IsInRoom;
use crate::direction::Direction;
use crate::entity::Entity;
use crate::game::Game;
use crate::map::builder::simple::Simple as SimpleMapBuilder;
use crate::parsing::error::Error;
use crate::parsing::parser::Parser;
use crate::player::Player;
use crate::world::World;

/// The `TwoWord` type.
///
/// This will actually be slightly more than a two-word parser, but who cares?
#[derive(Clone, Debug, Default)]
pub struct TwoWord {
  pub game: Game,
}

impl TwoWord {
  /// Constructor.
  pub fn new() -> Self {
    let map = (SimpleMapBuilder {}).build();
    let mut entities = HashMap::new();
    let mut entity = Entity::default();
    let spawn_room_id = map.spawn_room_id.clone().unwrap();
    entity.is_in_room = Some(IsInRoom(spawn_room_id));
    let entity_id = entity.id.clone();
    entities.insert(entity_id.clone(), entity);
    let player = Some(Player { entity_id });
    let world = Some(World {
      entities,
      map: Some(map),
    });
    let game = Game { player, world };
    Self { game }
  }
}

impl Parser for TwoWord {
  /// Parse two (or more) words of input.
  fn parse_input(&mut self, input: &str) -> Result<Option<String>, Error> {
    let words = input.split(' ').map(str::to_string).collect::<Vec<String>>();
    let word0 = words.get(0).cloned().unwrap_or_default();
    let word1 = words.get(1).cloned().unwrap_or_default();
    let result = match (word0.as_str(), word1.as_str()) {
      ("look", direction) if Direction::from_str(direction).is_ok() => {
        Command::LookDirection(Direction::from_str(direction)?).execute(&mut self.game)?
      },
      ("look", _) => Command::LookAround.execute(&mut self.game)?,
      ("go", direction) | (direction, _) if Direction::from_str(direction).is_ok() => {
        Command::GoDirection(Direction::from_str(direction)?).execute(&mut self.game)?
      },
      ("quit", _) => Command::Quit.execute(&mut self.game)?,
      (_, _) => todo!(),
    };
    Ok(result)
  }
}
