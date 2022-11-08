use crate::action::Action;

use super::*;

impl<'a> ProcessAction {
  pub fn process_look_at_object(&mut self, action: &Action, data: &mut ProcessActionData<'a>) {
    if let Action::LookAtObject { object_id, .. } = action {
      let object = get_entity!(data, object_id);
      info!("Sending event (description of indicated object).");
      data.output_event_channel.single_write(OutputEvent {
        string: format!(
          "You look at the {}...",
          get_name!(data, object).unwrap_or(&"<WTF>".to_owned()).to_lowercase()
        ),
      });
      data.output_event_channel.single_write(OutputEvent {
        string: get_brief_description!(data, object)
          .unwrap_or(&"<WTF>".to_string())
          .to_string(),
      });
    }
  }
}
