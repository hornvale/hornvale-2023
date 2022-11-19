use crate::ecs::entity::EntityId;
use crate::ecs::system::action_processor::Data as ActionProcessorData;
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
            write_effect_event!(
              data,
              Effect::EntityWalksOutOfRoom(EntityWalksOutOfRoom {
                entity_id: self.entity_id,
                direction: self.direction,
                room_id,
              })
            );
            write_effect_event!(
              data,
              Effect::EntityWalksIntoRoom(EntityWalksIntoRoom {
                entity_id: self.entity_id,
                direction: self.direction.get_inverse(),
                room_id: destination_id,
              })
            );
            write_effect_event!(
              data,
              Effect::EntityLooksAround(EntityLooksAround {
                entity_id: self.entity_id,
              })
            );
            write_effect_event!(
              data,
              Effect::EntitySetInitiative(EntitySetInitiative {
                entity_id: self.entity_id,
                value: 0,
              })
            );
          },
          _ => {
            you!(data, entity, "are unable to move in that direction!");
          },
        },
        None => {
          you!(data, entity, "are unable to move in that direction!");
        },
      }
    }
    Ok(())
  }
}
