use crate::ecs::entity::EntityId;
use crate::ecs::event_channels::*;
use crate::ecs::systems::action_processor::Data as ActionProcessorData;
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
    if entity_id_has_camera!(data, self.entity_id) {
      info!("Sending event (description of indicated entity).");
      data.output_event_channel.single_write(OutputEvent {
        string: format!(
          "You look at the {}...",
          get_name!(data, target_entity)
            .unwrap_or(&"<WTF>".to_owned())
            .to_lowercase()
        ),
      });
      data.output_event_channel.single_write(OutputEvent {
        string: get_brief_description!(data, target_entity).0.clone(),
      });
    }
    Ok(())
  }
}
