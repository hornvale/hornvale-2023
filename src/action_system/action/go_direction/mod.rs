use crate::direction::Direction;
use crate::entity::EntityId;
use crate::world::World;

use super::super::error::Error;

/// The `GoDirection` action.
pub struct GoDirection {
  /// Actor entity ID.
  pub entity_id: EntityId,
  /// Direction.
  pub direction: Direction,
}

impl GoDirection {
  pub fn execute(&self, _world: &mut World) -> Result<Option<String>, Error> {
    /*
    let mut entity = world.entities.get_mut(entity_id).unwrap();
    let room_id = &entity.is_in_room.as_ref().unwrap().0;
    let map = world.map.as_ref().unwrap();
    let room = map.rooms.get(room_id).unwrap();
    let passages = room.has_passages.as_ref().unwrap();
    match passages.get_passage_to(direction) {
      Some(Passage {
        destination: Destination::Room(room_id),
        ..
      }) => {
        entity.is_in_room = Some(IsInRoom(room_id.clone()));
        Some(format!("You walk to the {}.", direction.get_lowercase()))
      },
      _ => Some("You bump your noggin into the wall.".to_string()),
    }*/
    Ok(Some("TEST".to_string()))
  }
}
