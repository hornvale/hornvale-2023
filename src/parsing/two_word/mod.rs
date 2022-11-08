use crate::parsing::error::Error;
use crate::parsing::parser::Parser;

/// The `TwoWord` type.
///
/// This will actually be slightly more than a two-word parser, but who cares?
#[derive(Clone, Debug, Default)]
pub struct TwoWord {
  // pub game: Game,
}

impl TwoWord {
  /// Constructor.
  pub fn new() -> Self {
    /*
    let mut world = World::new();
    world.build_simple().unwrap();
    let spawn_room_id = world.spawn_room_id.clone();
    let entity = Entity::default();
    world.move_entity_to_room(&entity.id, &spawn_room_id).unwrap();
    let player = Player { entity_id: entity.id };
    let game = Game { player: Some(player), world: Some(world) };
    */
    Self {} //game }
  }
}

impl Parser for TwoWord {
  /// Parse two (or more) words of input.
  fn parse_input(&mut self, input: &str) -> Result<Option<String>, Error> {
    let words = input.split(' ').map(str::to_string).collect::<Vec<String>>();
    let word0 = words.get(0).cloned().unwrap_or_default();
    let word1 = words.get(1).cloned().unwrap_or_default();
    // let player_entity_id = self.game.player.as_ref().unwrap().entity_id.clone();
    /*
    match (word0.as_str(), word1.as_str()) {
      /*
      ("look", direction) if Direction::from_str(direction).is_ok() => {
        //Command::LookDirection(player_entity_id, Direction::from_str(direction)?).execute(&mut self.game)?
      },
      ("look", _) => Command::LookAround(player_entity_id).execute(&mut self.game)?,
      ("go", direction) | (direction, _) if Direction::from_str(direction).is_ok() => {
        Command::GoDirection(player_entity_id, Direction::from_str(direction)?).execute(&mut self.game)?
      },
      ("quit", _) => Command::Quit(player_entity_id).execute(&mut self.game)?,
      */
      (_, _) => todo!(),
    }
    */
    let (_, _) = (word0.as_str(), word1.as_str());
    todo!();
  }
}
