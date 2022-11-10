use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use super::super::components::*;
use super::super::entity::*;
use super::super::event_channels::*;
use super::super::resources::*;

mod get_command;
mod match_visible_entity;

pub struct ProcessInput {
  pub reader_id: ReaderId<InputEvent>,
}

impl ProcessInput {}

#[derive(SystemData)]
pub struct ProcessInputData<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Read<'a, PlayerResource>,
  pub output_resource: Write<'a, OutputResource>,
  pub command_event_channel: Write<'a, EventChannel<CommandEvent>>,
  pub input_event_channel: Read<'a, EventChannel<InputEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_description: ReadStorage<'a, HasDescription>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: ReadStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for ProcessInput {
  type SystemData = ProcessInputData<'a>;

  /// Run system.
  fn run(&mut self, mut data: Self::SystemData) {
    let input_events = data
      .input_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<InputEvent>>();
    let event_count = input_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} input event(s)...", event_count);
    for event in input_events.iter() {
      data.output_event_channel.single_write(OutputEvent {
        string: format!("> {}\n\n", event.input.trim()),
      });
      if let Ok(command) = self.get_command(&event.input, &mut data) {
        data.command_event_channel.single_write(CommandEvent { command });
      } else {
        error!("Something was screwed up about {:?}", event);
      }
    }
  }
}
