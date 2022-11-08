use specs::prelude::*;

use crate::action::Action;
use crate::map::PassageDestination;

use super::*;

impl<'a> ProcessAction {
  /// Process `look <direction>`, e.g. "look north".
  pub fn process_look_direction(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::LookDirection { entity_id, direction } = action {
      let entity = get_entity!(data, entity_id);
      if let Some(room_id) = get_current_room!(data, entity) {
        let room = get_entity!(data, room_id);
        match get_passage_to!(data, room, direction) {
          Some(passage) => match passage.to {
            PassageDestination::Room(destination_id) => {
              info!("Sending event (description of indicated room).");
              data.output_event_channel.single_write(OutputEvent {
                string: format!("You look to the {}...", &direction.get_lowercase()),
              });
              let destination_room = get_entity!(data, destination_id);
              data.output_event_channel.single_write(OutputEvent {
                string: format_room!(data, destination_room),
              });
            },
            _ => {
              data.output_event_channel.single_write(OutputEvent {
                string: "You are unable to look in that direction!".into(),
              });
            },
          },
          None => {
            data.output_event_channel.single_write(OutputEvent {
              string: "You are unable to look in that direction!".into(),
            });
          },
        }
      }
    }
  }
}
