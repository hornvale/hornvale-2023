use crate::entity::EntityId;
use crate::system::effect_processor::Data as EffectProcessorData;
use anyhow::Error;

/// `EntitySetInitiative`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SetInitiative {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The value to which the initiative should be set.
  pub value: usize,
}

impl SetInitiative {
  pub fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let initiative = get_has_initiative!(data, entity).unwrap();
    has_initiative!(data, entity, self.value, initiative.increment);
    Ok(())
  }
}
