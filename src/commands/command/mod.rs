// use crate::action_system::factory::Factory;
use crate::commands::error::Error;
use crate::direction::Direction;
use crate::entity::id::Id as EntityId;
use crate::game::Game;

/// The `Command` enum.
pub enum Command {
  /// IC: Look at surroundings.
  LookAround(EntityId),
  /// IC: Look in a specific direction.
  LookDirection(EntityId, Direction),
  /// IC: Go in a specific direction.
  GoDirection(EntityId, Direction),
  /// OOC: Quit.
  Quit(EntityId),
}

impl Command {
  /// Execute.

  pub fn execute(&mut self, game: &mut Game) -> Result<Option<String>, Error> {
    use Command::*;
    let _world = game.world.as_mut().unwrap();
    match &self {
      //LookAround(entity_id) => Action::LookAround(entity_id.clone()).execute(world)?,
      //GoDirection(entity_id, direction) => Action::GoDirection(entity_id.clone(), *direction).execute(world)?,
      //LookDirection(entity_id, direction) => Action::LookDirection(entity_id.clone(), *direction).execute(world)?,
      Quit(_entity_id) => Err(Error::UserExitError),
      _ => Err(Error::UserExitError),
    }
  }
}
