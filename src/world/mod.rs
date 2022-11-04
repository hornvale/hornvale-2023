use std::collections::HashMap;

use crate::entity::id::Id as EntityId;
use crate::entity::Entity;
use crate::map::Map;

/// The `World` struct.
#[derive(Clone, Debug, Default)]
pub struct World {
  /// The entities.
  pub entities: HashMap<EntityId, Entity>,
  /// The map of the world.
  pub map: Option<Map>,
}

impl World {
  /// Constructor.
  pub fn new() -> Self {
    let entities = HashMap::new();
    let map = None;
    Self { entities, map }
  }
}
