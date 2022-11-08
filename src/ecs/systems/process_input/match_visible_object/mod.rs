use anyhow::Error as AnyError;
use specs::prelude::*;

use super::*;

impl<'a> ProcessInput {
  /// Match any visible object with specific text.
  pub fn match_visible_object(&mut self, input: &str, data: &mut ProcessInputData<'a>) -> Result<Entity, AnyError> {
    info!(
      "Attempting to match a visible object with the descriptive text '{}'",
      input
    );
    let player_id = data.player_resource.0.unwrap();
    let player = data.entities.entity(player_id.0);
    let mut result = Err(anyhow!("Not found"));
    if let Some(current_room) = get_current_room!(data, player) {
      info!("Examining visible objects in room {:?}", current_room);
      if let Some((object_entity, _is_in_room, _is_an_object, _has_name, _has_description)) = (
        &data.entities,
        &data.is_in_room,
        &data.is_an_object,
        &data.has_name,
        &data.has_description,
      )
        .join()
        .filter(|(_entity, is_in_room, _is_an_object, _has_name, _has_description)| is_in_room.0 == current_room)
        .filter(|(_entity, _is_in_room, _is_an_object, has_name, _has_description)| {
          has_name.0.to_lowercase() == input.to_lowercase()
        })
        .collect::<Vec<_>>()
        .first()
      {
        info!("Found at least one candidate visible object: {:?}", _has_name);
        result = Ok(*object_entity);
      }
    }
    result
  }
}
