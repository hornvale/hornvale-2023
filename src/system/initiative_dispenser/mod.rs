use crate::component::*;
use crate::initiative::MAX_INITIATIVE;
use specs::prelude::*;

pub struct InitiativeDispenser {}

impl InitiativeDispenser {}

#[derive(SystemData)]
pub struct Data<'a> {
  pub entities: Entities<'a>,
  pub has_initiative: WriteStorage<'a, HasInitiative>,
}

impl<'a> System<'a> for InitiativeDispenser {
  type SystemData = Data<'a>;

  /// Run the system.
  fn run(&mut self, mut data: Self::SystemData) {
    for (_entity, mut has_initiative) in (&data.entities, &mut data.has_initiative).join() {
      has_initiative.0.current += has_initiative.0.increment;
      if has_initiative.0.current > MAX_INITIATIVE {
        has_initiative.0.current = MAX_INITIATIVE;
      }
    }
  }
}
