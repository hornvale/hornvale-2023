use crate::action::_trait::actionable::Actionable;
use crate::ecs::event::*;
use crate::ecs::AllData;
use specs::prelude::*;
use specs::shrev::ReaderId;

pub struct ActionProcessor {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ActionProcessor {}

impl<'a> System<'a> for ActionProcessor {
  type SystemData = AllData<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .action_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<ActionEvent>>();
    let event_count = events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} action event(s)...", event_count);
    for event in events.iter() {
      debug!("Processing next action event, {:?}", event);
      let ActionEvent { action } = event;
      match action.execute(&mut data) {
        Ok(()) => {},
        Err(error) => action_error!(data, action, error),
      }
    }
  }
}
