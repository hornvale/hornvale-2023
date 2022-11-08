use super::super::super::event_channels::OutputEvent;
use crate::command::Command;

use super::*;

impl<'a> ProcessCommand {
  /// Process an "echo" command.
  pub fn process_echo(&mut self, command: &Command, data: &mut ProcessCommandData<'a>) {
    if let Command::Echo { string, .. } = command {
      data.output_event_channel.single_write(OutputEvent {
        string: string.to_string(),
      });
    }
  }
}
