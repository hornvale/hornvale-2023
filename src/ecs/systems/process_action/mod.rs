use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use super::super::components::*;
use super::super::event_channels::*;
use super::super::resources::*;
use crate::action::Action;

mod go_direction;
mod look_around;
mod look_at_object;
mod look_direction;

pub struct ProcessAction {
  pub reader_id: ReaderId<ActionEvent>,
}

impl ProcessAction {}

#[derive(SystemData)]
pub struct ProcessActionData<'a> {
  pub entities: Entities<'a>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub has_name: ReadStorage<'a, HasName>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: WriteStorage<'a, IsInRoom>,
  pub player_resource: Read<'a, PlayerResource>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessAction {
  type SystemData = ProcessActionData<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let events = data
      .action_event_channel
      .read(&mut self.reader_id)
      .into_iter()
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
      use Action::*;
      match action {
        GoDirection { .. } => self.process_go_direction(action, &mut data),
        LookAround { .. } => {
          self.process_look_around(action, &mut data);
        },
        LookAtObject { .. } => self.process_look_at_object(action, &mut data),
        LookDirection { .. } => self.process_look_direction(action, &mut data),
      }
    }
  }
}
