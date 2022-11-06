use anyhow::Context;

use std::collections::HashMap;

use crate::entity::{Entity, EntityCollection, EntityId, EntityRoomMap};
use crate::map::Map;
use crate::room::{Room, RoomCollection, RoomId};

pub mod error;
use error::Error;

/// The `World` struct.
///
/// The world represents everything useful to know about the in-game world.
///
/// Therefore, it obviously runs the risk of becoming a God Object. It also
/// could become aggravatingly difficult to create test fixtures that adhere
/// to the trait.
///
/// I think a reasonable alternative, therefore, is to use a Service Container
/// or Service Locator pattern, and treat the World as the container... with a
/// small amount of sugar for comfort.
#[derive(Clone, Debug, Default)]
pub struct World {
  /// The entities.
  pub entity_collection: EntityCollection,
  /// The entity->room map.
  pub entity_room_map: EntityRoomMap,
  /// The rooms.
  pub room_collection: RoomCollection,
  /// The map of the world.
  pub map: Option<Map>,
}

impl World {
  /// Constructor.
  pub fn new() -> Self {
    let entity_collection = EntityCollection(HashMap::new());
    let entity_room_map = EntityRoomMap(HashMap::new());
    let room_collection = RoomCollection(HashMap::new());
    let map = None;
    Self {
      entity_collection,
      entity_room_map,
      room_collection,
      map,
    }
  }

  /// Retrieve the Entity Collection.
  pub fn get_entity_collection(&self) -> &EntityCollection {
    &self.entity_collection
  }

  /// Retrieve the Entity Collection _mutably_.
  pub fn get_entity_collection_mut(&mut self) -> &mut EntityCollection {
    &mut self.entity_collection
  }

  /// Get a specific entity by its ID.
  pub fn get_entity(&self, entity_id: &EntityId) -> Result<&Entity, Error> {
    let entity = self.entity_collection.0.get(entity_id).context("entity not found")?;
    Ok(entity)
  }

  /// Retrieve the Room Collection.
  pub fn get_room_collection(&self) -> &RoomCollection {
    &self.room_collection
  }

  /// Retrieve the Room Collection _mutably_.
  pub fn get_room_collection_mut(&mut self) -> &mut RoomCollection {
    &mut self.room_collection
  }

  /// Get a specific room by its ID.
  pub fn get_room(&self, room_id: &RoomId) -> Result<&Room, Error> {
    Ok(self.room_collection.0.get(room_id).context("room not found")?)
  }

  /// Retrieve the Entity->Room Map.
  pub fn get_entity_room_map(&self) -> &EntityRoomMap {
    &self.entity_room_map
  }

  /// Retrieve the Entity->Room Map _mutably_.
  pub fn get_entity_room_map_mut(&mut self) -> &mut EntityRoomMap {
    &mut self.entity_room_map
  }

  /// Get the ID of the room in which a specific entity is, by its ID.
  pub fn get_entity_room_id(&self, entity_id: &EntityId) -> Result<&RoomId, Error> {
    Ok(self.entity_room_map.0.get(entity_id).context("entity not found")?)
  }

  /// Get the description of the room with a specified ID.
  pub fn get_room_description(&self, room_id: &RoomId) -> Result<String, Error> {
    Ok(format!("{} looks like crap", room_id))
  }
}
