use crate::ecs::entity::EntityId;
use crate::ecs::entity::RoomId;
use crate::ecs::system::effect_processor::Data as EffectProcessorData;
use crate::effect::Effectable;
use crate::map::Direction;
use anyhow::Error;

/// `EntityWalksIntoRoom`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntoRoom {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The room the entity enters.
  pub room_id: RoomId,
  /// The direction from which the entity enters.
  pub direction: Direction,
}

impl Effectable for IntoRoom {
  fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let name = get_name!(data, entity).unwrap();
    is_in_room!(data, entity, self.room_id);
    they!(
      data,
      entity,
      format!("{} walks in from the {}.", name, self.direction.get_lowercase())
    );
    reset_state!(data, entity, 0);
    set_state!(data, entity, 1);
    Ok(())
  }
}
