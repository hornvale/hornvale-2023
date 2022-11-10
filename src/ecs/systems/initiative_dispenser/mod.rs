use super::super::components::*;
use super::super::resources::*;
use specs::prelude::*;

pub struct InitiativeDispenser {}

impl InitiativeDispenser {}

#[derive(SystemData)]
pub struct InitiativeDispenserData<'a> {
  pub entities: Entities<'a>,
  pub tick_resource: Write<'a, TickResource>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
}

impl<'a> System<'a> for InitiativeDispenser {
  type SystemData = InitiativeDispenserData<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    if data.tick_resource.0 % 10 == 0 {
      return;
    }
    for (_entity, mut has_initiative) in (&data.entities, &mut data.has_initiative).join() {
      has_initiative.current += has_initiative.refill_rate;
    }
  }
}
