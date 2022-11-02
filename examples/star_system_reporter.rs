#![allow(unused_imports)]
use hornvale::astronomy::star_system::constraints::Constraints;
use hornvale::astronomy::star_system::error::Error;
use hornvale::astronomy::star_system::StarSystem;
///! Generates a star system and prints a little report on it.
use hornvale::*;
use rand::prelude::*;

#[macro_use]
extern crate function_name;

pub struct StarSystemReporter {}

impl StarSystemReporter {
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
  let constraints = Constraints::main_sequence();
  let star_system = constraints.generate(&mut rng)?;
  let reporter = StarSystemReporter::new();
  reporter.report(&star_system);

  Ok(())
}
