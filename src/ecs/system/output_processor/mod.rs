use crate::ecs::event::OutputEvent;
use crate::ecs::resource::*;
use crate::formatting::format_string;
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};
use std::io::Write as _;

pub struct OutputProcessor {
  pub reader_id: ReaderId<OutputEvent>,
}

impl OutputProcessor {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub output_resource: Write<'a, OutputResource>,
  pub output_event_channel: Read<'a, EventChannel<OutputEvent>>,
}

impl<'a> System<'a> for OutputProcessor {
  type SystemData = Data<'a>;

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
      let string = format_string(event.string.trim());
      writeln!(output, "{}\n", string).unwrap();
    }
  }
}
