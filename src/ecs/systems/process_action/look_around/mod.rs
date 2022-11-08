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
      }
    }
  }
}
