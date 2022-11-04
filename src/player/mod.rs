use crate::entity::id::Id as EntityId;

/// The `Player` type.
#[derive(Clone, Debug)]
pub struct Player {
  /// The entity this object encompasses.
  pub entity_id: EntityId,
}

impl Player {
  pub fn new(entity_id: EntityId) -> Self {
    Self { entity_id }
  }
}
