use crate::actions::action::Action;
use crate::commands::error::Error;
use crate::direction::Direction;
use crate::game::Game;

/// The `Command` enum.
pub enum Command {
  /// Look at surroundings.
  LookAround,
  /// Look in a specific direction.
  LookDirection(Direction),
  /// Go in a specific direction.
  GoDirection(Direction),
  /// Quit.
  Quit,
}

impl Command {
  /// Execute.

  pub fn execute(&mut self, game: &mut Game) -> Result<Option<String>, Error> {
    use Command::*;
    let player_entity_id = &game.player.as_ref().unwrap().entity_id.clone();
    let world = game.world.as_mut().unwrap();
    let result = match &self {
      LookAround => Action::LookAround.execute(player_entity_id, world)?,
      GoDirection(direction) => Action::GoDirection(*direction).execute(player_entity_id, world)?,
      LookDirection(direction) => Action::LookDirection(*direction).execute(player_entity_id, world)?,
      Quit => return Err(Error::UserExitError),
    };
    Ok(result)
  }
}
