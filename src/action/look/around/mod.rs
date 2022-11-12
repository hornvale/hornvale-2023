use crate::entity::EntityId;
use crate::event::*;
use crate::system::action_processor::Data as ActionProcessorData;
use anyhow::Error;
use specs::prelude::*;

/// The `LookAround` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAround {
  pub entity_id: EntityId,
}

impl LookAround {
  pub fn execute(&self, data: &mut ActionProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    if let Some(room_id) = get_current_room_id!(data, entity) {
      let room = get_entity!(data, room_id);
      if entity_id_has_camera!(data, self.entity_id) {
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
    Ok(())
  }
}
