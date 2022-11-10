use crate::action::Action;

use super::*;

impl<'a> ProcessAction {
  pub fn process_look_at_entity(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::LookAtEntity {
      entity_id,
      target_entity_id,
      ..
    } = action
    {
      let target_entity = get_entity!(data, target_entity_id);
      if has_camera!(data, entity_id) {
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
          string: get_brief_description!(data, target_entity)
            .unwrap_or(&"<WTF>".to_string())
            .to_string(),
        });
      }
    }
  }
}
