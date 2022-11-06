use std::collections::HashMap;

use super::{Entity, EntityId};

/// The `EntityCollection` type.
#[derive(Clone, Debug, Default)]
pub struct Collection(pub HashMap<EntityId, Entity>);

impl Collection {
  /// Get the inner hashmap representation.
  pub fn get(&self) -> &HashMap<EntityId, Entity> {
    &self.0
  }

  /// Get the inner hashmap representation.
  pub fn get_mut(&mut self) -> &mut HashMap<EntityId, Entity> {
    &mut self.0
  }
}
