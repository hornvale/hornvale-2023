use crate::entity::EntityId;
use crate::system::action_processor::Data as ActionProcessorData;
use anyhow::Error;

/// The `LookAtEntity` command.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LookAtEntity {
  pub entity_id: EntityId,
  pub target_entity_id: EntityId,
}

impl LookAtEntity {
  pub fn execute(&self, data: &mut ActionProcessorData) -> Result<(), Error> {
    let target_entity = get_entity!(data, self.target_entity_id);
    info!("Sending event (description of indicated entity).");
    let entity = get_entity!(data, self.entity_id);
    you!(
      data,
      entity,
      format!("look at the {}...", get_lc_name!(data, target_entity).unwrap())
    );
    if entity_id_has_camera!(data, self.entity_id) {
      write_output_event!(data, get_brief_description!(data, target_entity).unwrap().0.clone());
    }
    Ok(())
  }
}
