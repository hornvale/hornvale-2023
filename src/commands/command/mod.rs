use crate::commands::error::Error;
use crate::direction::Direction;
use crate::player::Player;

/// The `Command` enum.
pub enum Command {
  /// Look at surroundings.
  Look(Player),
  /// Look in a specific direction.
  LookDirection(Player, Direction),
  /// Go in a specific direction.
  GoDirection(Player, Direction),
  /// Quit.
  Quit(Player),
}

impl Command {
  /// Execute.

  pub fn execute(&mut self) -> Result<Option<String>, Error> {
    use Command::*;
    let result = match &self {
      Look(_player) => Some("You see a lot of WTF.".to_owned()),
      GoDirection(_player, direction) => Some(format!(
        "You can't go {} yet (you're not smart enough).",
        format!("{}", direction).to_lowercase()
      )),
      LookDirection(_player, direction) => Some(format!(
        "You can't look {} yet (you're not smart enough).",
        format!("{}", direction).to_lowercase()
      )),
      Quit(_player) => return Err(Error::UserExitError),
    };
    Ok(result)
  }
}
