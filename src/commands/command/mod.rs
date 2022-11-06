use crate::action_system::factory::Factory as ActionFactory;
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

  pub fn execute(self, game: &mut Game) -> Result<Option<String>, Error> {
    use Command::*;
    let world = game.world.as_mut().unwrap();
    match &self {
      GoDirection(ref entity_id, ref direction) => {
        let action = ActionFactory::go_direction(entity_id.clone(), *direction);
        let result = action.execute(world)?;
        Ok(result)
      },
      LookAround(ref entity_id) => {
        let action = ActionFactory::look_around(entity_id.clone());
        let result = action.execute(world)?;
        Ok(result)
      },
      LookDirection(ref entity_id, ref direction) => {
        let action = ActionFactory::look_direction(entity_id.clone(), *direction);
        let result = action.execute(world)?;
        Ok(result)
      },
      Quit(_entity_id) => Err(Error::UserExitError),
    }
  }
}
