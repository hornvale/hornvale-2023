use crate::commands::error::Error;
use crate::direction::Direction;
use crate::player::Player;

/// The `Command` enum.
pub enum Command {
  /// Look at surroundings.
  Look(Player),
  /// Go in a specific direction.
  Go(Player, Direction),
}

impl Command {
  /// Execute.

  pub fn execute(&mut self) -> Result<Option<String>, Error> {
    use Command::*;
    let result = match &self {
      Look(_player) => Some("You see a lot of WTF.".to_owned()),
      Go(_player, _direction) => Some("You can't go in that direction yet (you're not smart enough).".to_owned()),
    };
    Ok(result)
  }
}
