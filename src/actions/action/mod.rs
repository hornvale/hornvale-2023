use crate::actions::error::Error;
use crate::direction::Direction;
use crate::entity::Entity;

/// The `Action` enum.
pub enum Action {
  /// Look around at current surroundings.
  LookAround,
  /// Look in a specific direction.
  LookDirection(Direction),
  /// Go in a specific direction.
  GoDirection(Direction),
}

impl Action {
  /// Execute.
  pub fn execute(&mut self, _entity: &Entity) -> Result<Option<String>, Error> {
    use Action::*;
    let result = match &self {
      LookAround => Some("You see a lot of WTF.".to_owned()),
      GoDirection(direction) => Some(format!(
        "You can't go {} yet (you're not smart enough).",
        format!("{}", direction).to_lowercase()
      )),
      LookDirection(direction) => Some(format!(
        "You can't look {} yet (you're not smart enough).",
        format!("{}", direction).to_lowercase()
      )),
    };
    Ok(result)
  }
}
