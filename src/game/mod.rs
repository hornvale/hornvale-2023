use crate::player::Player;
use crate::world::World;

/// The `Game` struct.
///
/// This gets passed around a lot.
#[derive(Clone, Debug, Default)]
pub struct Game {
  /// The world state, generally all of the information available.
  pub world: Option<World>,
  /// The player.
  pub player: Option<Player>,
}
