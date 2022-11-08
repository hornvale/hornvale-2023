use super::super::super::event_channels::ActionEvent;
use crate::action::Action;

use super::*;

impl<'a> ProcessCommand {
  /// Process a command that transforms into an action.
  pub fn process_action(&mut self, action: Action, data: &mut ProcessCommandData<'a>) {
    data.action_event_channel.single_write(ActionEvent { action });
  }
}
