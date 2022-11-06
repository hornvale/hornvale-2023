use crate::world::World;

use super::action::{go_direction::GoDirection, look_around::LookAround, look_direction::LookDirection};
use super::error::Error;

/// The `Factory` enum.
///
/// Actions define in-character efforts to query or affect the game world in
/// some way.  Thus, we must pass the World.
///
/// Normally, I think this will be an Enum-based factory.
pub enum Factory {
  /// Look around at current surroundings.
  LookAround(LookAround),
  /// An entity looks in a specific direction.
  LookDirection(LookDirection),
  /// Go in a specific direction.
  GoDirection(GoDirection),
}

impl Factory {
  /// Execute.
  pub fn execute(&self, world: &mut World) -> Result<Option<String>, Error> {
    match &self {
      Factory::GoDirection(action) => action.execute(world),
      Factory::LookAround(action) => action.execute(world),
      Factory::LookDirection(action) => action.execute(world),
    }
  }
}
