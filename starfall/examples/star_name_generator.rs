#![allow(unused_imports)]

use rand::prelude::*;
use starfall::astronomy::star::name::{generate_star_name, INFIX, PREFIX, SUFFIX};
use starfall::*;

#[named]
fn main() {
  init_pretty_env_logger();
  trace_enter!();
  let mut rng = thread_rng();
  trace_var!(rng);
  println!(
    "Generating names from {} possibilities...",
    PREFIX.len() * INFIX.len() * SUFFIX.len()
  );
  for _ in 1..20 {
    let name = generate_star_name(&mut rng);
    trace_var!(name);
    print_var!(name);
  }
  trace_exit!();
}
