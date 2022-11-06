use crate::entity::EntityId;
use crate::world::World;

use super::super::error::Error;

/// The `LookDirection` action.
pub struct LookDirection {
  /// Actor entity ID.
  pub entity_id: EntityId,
}

impl LookDirection {
  pub fn execute(&self, _world: &mut World) -> Result<Option<String>, Error> {
    Ok(Some("WHEE!".to_string()))
  }
}

/*{
/*
let entity = world.entities.get_mut(entity_id).unwrap();
let room_id = &entity.is_in_room.as_ref().unwrap().0;
let map = world.map.as_ref().unwrap();
let room = map.rooms.get(room_id).unwrap();
let passages = room.has_passages.as_ref().unwrap();
match passages.get_passage_to(direction) {
  Some(Passage {
    destination: Destination::Room(room_id),
    ..
  }) => {
    let room = map.rooms.get(room_id).unwrap();
    let room_description = &room.has_description.as_ref().unwrap().brief;
    Some(format!(
      "You look to the {}.\n\n{}",
      direction.get_lowercase(),
      room_description
    ))
  },
  _ => Some(format!(
    "You don't see any passages leading {}.",
    direction.get_lowercase()
  )),
}
*/
Ok(Some("TEST".to_string()))
},*/
