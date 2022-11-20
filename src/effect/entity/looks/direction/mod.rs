use crate::ecs::entity::EntityId;
use crate::ecs::system::effect_processor::Data as EffectProcessorData;
use crate::map::{Direction, PassageDestination};
use anyhow::Error;

/// `EntityLooksDirection`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LooksDirection {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The direction the entity looks.
  pub direction: Direction,
}

impl LooksDirection {
  pub fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let room_id = get_current_room_id!(data, entity).unwrap();
    let room = get_entity!(data, room_id);
    let name = get_name!(data, entity).unwrap();
    use PassageDestination::*;
    if let Some(passage) = get_passage_to!(data, room, &self.direction) {
      match passage.to {
        Room(destination_id) => {
          let lc_direction = self.direction.get_lowercase();
          let destination_room = get_entity!(data, destination_id);
          you!(
            data,
            entity,
            format!(
              "look to the {}...\n{}",
              lc_direction,
              format_room!(data, destination_room)
            )
          );
          they!(data, entity, format!("{} looks {}.", name, lc_direction));
          set_state!(data, entity, 0);
          reset_state!(data, entity, 1);
        },
        Message(_) => {},
      }
    }
    Ok(())
  }
}
