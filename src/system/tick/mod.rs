use specs::prelude::*;

use crate::resource::*;

pub struct Tick {}

#[derive(SystemData)]
pub struct TickData<'a> {
  pub entities: Entities<'a>,
  pub tick_resource: Write<'a, TickResource>,
}

impl<'a> System<'a> for Tick {
  type SystemData = TickData<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    data.tick_resource.0 = data.tick_resource.0.wrapping_add(1);
  }
}
