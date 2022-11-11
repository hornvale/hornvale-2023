use crate::action::Action;
use crate::action::LookAroundAction;
use crate::ecs::entity::EntityId;
use crate::ecs::event_channels::*;
use crate::ecs::systems::action_processor::Data as ActionProcessorData;
use crate::effect::*;
use crate::map::Direction;
use crate::map::PassageDestination;
use anyhow::Error;

/// The `GoDirection` action.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GoDirection {
  pub entity_id: EntityId,
  pub direction: Direction,
}

impl GoDirection {
  pub fn execute(&self, data: &mut ActionProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    if let Some(room_id) = get_current_room_id!(data, entity) {
      let room_entity = get_entity!(data, room_id);
      match get_passage_to!(data, room_entity, &self.direction) {
        Some(passage) => match passage.to {
          PassageDestination::Room(destination_id) => {
            set_current_room_id!(data, entity, get_entity!(data, destination_id));
            data.effect_event_channel.single_write(EffectEvent {
              effect: Effect::EntityWalksOutOfRoom(EntityWalksOutOfRoom {
                entity_id: self.entity_id,
                direction: self.direction,
                room_id,
              }),
            });
            data.effect_event_channel.single_write(EffectEvent {
              effect: Effect::EntityWalksIntoRoom(EntityWalksIntoRoom {
                entity_id: self.entity_id,
                direction: self.direction.get_inverse(),
                room_id: destination_id,
              }),
            });
            data.action_event_channel.single_write(ActionEvent {
              action: Action::LookAround(LookAroundAction {
                entity_id: self.entity_id,
              }),
            });
          },
          _ => {
            if entity_id_has_camera!(data, self.entity_id) {
              write_output!(data, "You are unable to move in that direction!".to_string());
            }
          },
        },
        None => {
          if entity_id_has_camera!(data, self.entity_id) {
            write_output!(data, "You are unable to move in that direction!".to_string());
          }
        },
      }
    }
    Ok(())
  }
}
