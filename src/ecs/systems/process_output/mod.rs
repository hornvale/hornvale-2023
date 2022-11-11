use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};
use std::io::Write as _;

use super::super::event_channels::OutputEvent;
use super::super::resources::*;

pub mod format_string;

pub struct ProcessOutput {
  pub reader_id: ReaderId<OutputEvent>,
}

impl ProcessOutput {}

#[derive(SystemData)]
pub struct ProcessOutputData<'a> {
  pub entities: Entities<'a>,
  pub random_resource: Write<'a, RandomResource>,
  pub output_resource: Write<'a, OutputResource>,
  pub output_event_channel: Read<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for ProcessOutput {
  type SystemData = ProcessOutputData<'a>;

  /// Run the system.
  fn run(&mut self, data: Self::SystemData) {
    let output_events = data
      .output_event_channel
      .read(&mut self.reader_id)
      .collect::<Vec<&OutputEvent>>();
    let event_count = output_events.len();
    if event_count == 0 {
      return;
    }
    let mut output = clone_output!(data);
    info!("Processing {} output event(s)...", event_count);
    for event in output_events.iter() {
      let string = self.format_string(event.string.trim());
      writeln!(output, "{}\n", string).unwrap();
    }
  }
}
