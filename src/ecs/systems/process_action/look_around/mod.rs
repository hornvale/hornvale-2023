use specs::prelude::*;

use crate::action::Action;

use super::*;

impl<'a> ProcessAction {
  /// Look around.
  pub fn process_look_around(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::LookAround { entity_id } = action {
      let entity = get_entity!(data, entity_id);
      if let Some(room_id) = get_current_room!(data, entity) {
        let room = get_entity!(data, room_id);
        info!("Sending event (description of current room).");
        data.output_event_channel.single_write(OutputEvent {
          string: format_room!(data, room),
        });
        let tile_map_resource_option = &mut data.tile_map_resource.0;
        if let Some(ref mut tile_map) = tile_map_resource_option {
          let room_coords = tile_map.room_coords_map.get(&room_id).cloned().unwrap();
          tile_map.player_coordinates = Some(room_coords);
          tile_map.mark_visible(room_coords.0, room_coords.1);
          data.output_event_channel.single_write(OutputEvent {
            string: format!("{}", data.tile_map_resource.0.as_ref().unwrap()),
          });
        }
      }
    }
  }
}
