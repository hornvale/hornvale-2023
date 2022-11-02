#![allow(unused_imports)]
///! Generates a star system and prints a little report on it.
use hornvale::astronomy::star_system::constraints::Constraints;
use hornvale::astronomy::star_system::error::Error;
use hornvale::astronomy::star_system::StarSystem;
use hornvale::*;
use rand::prelude::*;

#[macro_use]
extern crate function_name;

pub struct HabitablePlanetReporter {}

impl HabitablePlanetReporter {
  pub fn new() -> Self {
    Self {}
  }

  pub fn report(&self, star_system: &StarSystem) {
    print_var!(star_system);
  }
}

fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  let mut rng = rand::thread_rng();
  let constraints = Constraints::habitable();
  let mut star_system = constraints.generate(&mut rng)?;
  let mut is_habitable = star_system.is_habitable();
  let mut counter = 0;
  while !is_habitable && counter < 50 {
    star_system = constraints.generate(&mut rng)?;
    is_habitable = star_system.is_habitable();
    counter += 1;
  }
  let reporter = HabitablePlanetReporter::new();
  reporter.report(&star_system);

  Ok(())
}
