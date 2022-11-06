use crate::entity::EntityId;
use crate::world::World;

use super::super::error::Error;

/// The `LookAround` action.
pub struct LookAround {
  /// Actor entity ID.
  pub entity_id: EntityId,
}

impl LookAround {
  pub fn execute(&self, world: &mut World) -> Result<Option<String>, Error> {
    let room_id = world.get_entity_room_id(&self.entity_id)?;
    let description = world.get_room_description(room_id)?;
    Ok(Some(description))
  }
}

/*
{
/*
  let entity = world.entities.get(entity_id).unwrap();
  let room_id = &entity.is_in_room.as_ref().unwrap().0;
  let map = world.map.as_ref().unwrap();
  let room = map.rooms.get(room_id).unwrap();
  let room_description = &room.has_description.as_ref().unwrap().brief;
  Some(room_description.to_string())
  */
  Ok(Some("TEST!".to_string()))
},
*/
