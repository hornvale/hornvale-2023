use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use crate::command::Command;

use super::super::event_channels::*;

mod action;
mod echo;
mod quit;

pub struct ProcessCommand {
  pub reader_id: ReaderId<CommandEvent>,
}

impl ProcessCommand {}

#[derive(SystemData)]
pub struct ProcessCommandData<'a> {
  pub entities: Entities<'a>,
  pub action_event_channel: Write<'a, EventChannel<ActionEvent>>,
  pub command_event_channel: Read<'a, EventChannel<CommandEvent>>,
  pub output_event_channel: Write<'a, EventChannel<OutputEvent>>,
  // pub script_event_channel: Write<'a, EventChannel<ScriptEvent>>,
}

impl<'a> System<'a> for ProcessCommand {
  type SystemData = ProcessCommandData<'a>;

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
      use Command::*;
      match command {
        Echo { .. } => self.process_echo(command, &mut data),
        Quit { .. } => self.process_quit(),
        /*
        Eval { string, .. } => data.script_event_channel.single_write(ScriptEvent {
          script: string.to_string(),
        }),
        */
        _ => {
          if let Ok(action) = command.get_action() {
            info!("Calculated action {:?} for command {:?}...", action, command);
            self.process_action(action, &mut data);
          } else {
            data.output_event_channel.single_write(OutputEvent {
              string: "I couldn't turn that command into an action.".to_string(),
            });
          }
        },
      }
    }
  }
}
