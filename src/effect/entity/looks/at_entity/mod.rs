use crate::ecs::entity::EntityId;
use crate::ecs::system::effect_processor::Data as EffectProcessorData;
use anyhow::Error;

/// `EntityLooksAtEntity`.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LooksAtEntity {
  /// The entity performing the action.
  pub entity_id: EntityId,
  /// The target entity.
  pub target_entity_id: EntityId,
}

impl LooksAtEntity {
  pub fn process(&self, data: &mut EffectProcessorData) -> Result<(), Error> {
    let entity = get_entity!(data, self.entity_id);
    let target_entity = get_entity!(data, self.target_entity_id);
    let actor_name = get_lc_name!(data, entity).unwrap();
    let lc_name = get_lc_name!(data, target_entity).unwrap();
    you!(data, entity, format!("look at {}...", lc_name));
    they!(data, entity, format!("{} looks at {}.", actor_name, lc_name));
    let brief = get_brief_description!(data, target_entity).unwrap().0.clone();
    show!(data, entity, brief);
    Ok(())
  }
}
