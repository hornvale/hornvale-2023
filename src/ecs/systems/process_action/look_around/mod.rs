use specs::prelude::*;

use crate::action::Action;

use super::*;

impl<'a> ProcessAction {
  /// Look around.
  pub fn process_look_around(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::LookAround { entity_id } = action {
      let entity = get_entity!(data, entity_id);
      if let Some(room_id) = get_current_room!(data, entity) {
        let room_coords = data
          .tile_map_resource
          .0
          .as_ref()
          .unwrap()
          .room_coords_map
          .get(&room_id)
          .cloned();
        data.tile_map_resource.0.as_mut().unwrap().player_coordinates = room_coords;
        let room = get_entity!(data, room_id);
        info!("Sending event (description of current room).");
        data.output_event_channel.single_write(OutputEvent {
          string: format_room!(data, room),
        });
        data.output_event_channel.single_write(OutputEvent {
          string: format!("{}", data.tile_map_resource.0.as_ref().unwrap()),
        });
      }
    }
  }
}
