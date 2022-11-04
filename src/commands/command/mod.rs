use crate::actions::action::Action;
use crate::commands::error::Error;
use crate::direction::Direction;
use crate::player::Player;

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

  pub fn execute(&mut self, player: &Player) -> Result<Option<String>, Error> {
    use Command::*;
    let result = match &self {
      LookAround => Action::LookAround.execute(&player.entity)?,
      GoDirection(direction) => Action::GoDirection(*direction).execute(&player.entity)?,
      LookDirection(direction) => Action::LookDirection(*direction).execute(&player.entity)?,
      Quit => return Err(Error::UserExitError),
    };
    Ok(result)
  }
}
