use crate::action::Action;
use crate::map::PassageDestination;

use super::super::super::entity::RoomId;
use super::*;

impl<'a> ProcessAction {
  /// Attempt to go in a specified direction.
  pub fn process_go_direction(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::GoDirection { entity_id, direction } = action {
      let entity = get_entity!(data, entity_id);
      if let Some(room_id) = get_current_room!(data, entity) {
        let room_entity = get_entity!(data, room_id);
        match get_passage_to!(data, room_entity, direction) {
          Some(passage) => match passage.to {
            PassageDestination::Room(destination_id) => {
              set_current_room!(data, entity, get_entity!(data, destination_id));
              data.action_event_channel.single_write(ActionEvent {
                action: Action::LookAround { entity_id: *entity_id },
              });
            },
            _ => {
              if has_camera!(data, entity_id) {
                data.output_event_channel.single_write(OutputEvent {
                  string: "You are unable to move in that direction!".into(),
                });
              }
            },
          },
          None => {
            if has_camera!(data, entity_id) {
              data.output_event_channel.single_write(OutputEvent {
                string: "You are unable to move in that direction!".into(),
              });
            }
          },
        }
      }
    }
  }
}
