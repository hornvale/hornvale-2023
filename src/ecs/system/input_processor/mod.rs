use crate::ecs::component::*;
use crate::ecs::entity::*;
use crate::ecs::event::*;
use crate::ecs::resource::*;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

mod get_command;
mod match_visible_entity;

pub struct InputProcessor {
  pub reader_id: ReaderId<InputEvent>,
}

impl InputProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub player_resource: Read<'a, PlayerResource>,
  pub output_resource: Write<'a, OutputResource>,
  pub command_event_channel: Write<'a, EventChannel<CommandEvent>>,
  pub input_event_channel: Read<'a, EventChannel<InputEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  pub has_brief_description: ReadStorage<'a, HasBriefDescription>,
  pub has_name: ReadStorage<'a, HasName>,
  pub has_passages: ReadStorage<'a, HasPassages>,
  pub is_a_player: ReadStorage<'a, IsAPlayer>,
  pub is_an_object: ReadStorage<'a, IsAnObject>,
  pub is_in_room: ReadStorage<'a, IsInRoom>,
}

impl<'a> System<'a> for InputProcessor {
  type SystemData = Data<'a>;

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
      if let Ok(command) = self.get_command(&event.input, &mut data) {
        write_command_event!(data, command);
      } else {
        error!("Something was screwed up about {:?}", event);
      }
    }
  }
}
