use std::collections::HashMap;

use super::{Room, RoomId};

/// The `RoomCollection` type.
#[derive(Clone, Debug, Default)]
pub struct Collection(pub HashMap<RoomId, Room>);

impl Collection {
  /// Get the inner hashmap representation.
  pub fn get(&self) -> &HashMap<RoomId, Room> {
    &self.0
  }

  /// Get the inner hashmap representation.
  pub fn get_mut(&mut self) -> &mut HashMap<RoomId, Room> {
    &mut self.0
  }
}
