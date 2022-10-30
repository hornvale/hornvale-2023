#![allow(unused_imports)]
use rand::prelude::*;
use starfall::astronomy::star_system::constraints::Constraints;
use starfall::astronomy::star_system::error::Error;
use starfall::astronomy::star_system::StarSystem;
///! Generates a star system and prints a little report on it.
use starfall::*;

pub struct StarSystemReporter {}

impl StarSystemReporter {
  pub fn new() -> Self {
    Self {}
  }

  pub fn report(&self, star_system: &StarSystem) {
    print_var!(star_system);
  }
}

#[named]
fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  trace_enter!();
  let mut rng = rand::thread_rng();
  let constraints = Constraints::main_sequence();
  let star_system = constraints.generate(&mut rng)?;
  let reporter = StarSystemReporter::new();
  reporter.report(&star_system);
  trace_exit!();
  Ok(())
}
