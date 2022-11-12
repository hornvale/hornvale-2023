use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::event::*;

pub struct CommandProcessor {
  pub reader_id: ReaderId<CommandEvent>,
}

impl CommandProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub command_event_channel: Read<'a, EventChannel<CommandEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  // pub script_event_channel: Write<'a, EventChannel<ScriptEvent>>,
}

impl<'a> System<'a> for CommandProcessor {
  type SystemData = Data<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    let command_events = data
      .command_event_channel
      .read(&mut self.reader_id)
      .cloned()
      .collect::<Vec<CommandEvent>>();
    let event_count = command_events.len();
    if event_count == 0 {
      return;
    }
    info!("Processing {} command event(s)...", event_count);
    for event in command_events.iter() {
      debug!("Processing next command event {:?}", event);
      let CommandEvent { command } = event;
      match command.get_action(&mut data) {
        Ok(Some(action)) => {
          info!("Calculated action {:?} for command {:?}...", action, command);
          data.action_event_channel.single_write(ActionEvent { action });
        },
        Ok(None) => {
          info!("Processed extradiagetic command {:?}...", command);
        },
        Err(error) => data.output_event_channel.single_write(OutputEvent {
          string: format!("encountered an error ({})", error),
        }),
      }
    }
  }
}
