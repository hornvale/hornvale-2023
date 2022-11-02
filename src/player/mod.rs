use crate::entity::Entity;

/// The `Player` type.
#[derive(Clone, Debug)]
pub struct Player {
  /// The entity this object encompasses.
  pub entity: Entity,
}

impl Player {
  pub fn new(entity: Entity) -> Self {
    Self { entity }
  }
}
