use crate::actions::action::Action;
use crate::commands::error::Error;
use crate::direction::Direction;
use crate::entity::id::Id as EntityId;
use crate::game::Game;

/// The `Command` enum.
pub enum Command {
  /// Look at surroundings.
  LookAround(EntityId),
  /// Look in a specific direction.
  LookDirection(EntityId, Direction),
  /// Go in a specific direction.
  GoDirection(EntityId, Direction),
  /// Quit.
  Quit(EntityId),
}

impl Command {
  /// Execute.

  pub fn execute(&mut self, game: &mut Game) -> Result<Option<String>, Error> {
    use Command::*;
    let world = game.world.as_mut().unwrap();
    let result = match &self {
      LookAround(entity_id) => Action::LookAround(entity_id.clone()).execute(world)?,
      GoDirection(entity_id, direction) => Action::GoDirection(entity_id.clone(), *direction).execute(world)?,
      LookDirection(entity_id, direction) => Action::LookDirection(entity_id.clone(), *direction).execute(world)?,
      Quit(_entity_id) => return Err(Error::UserExitError),
    };
    Ok(result)
  }
}
