#![allow(unused_imports)]

#[macro_use]
extern crate function_name;

use hornvale::astronomy::star::name::{generate_star_name, INFIX, PREFIX, SUFFIX};
use hornvale::*;
use rand::prelude::*;

fn main() {
  init_pretty_env_logger();

  let mut rng = thread_rng();

  println!(
    "Generating names from {} possibilities...",
    PREFIX.len() * INFIX.len() * SUFFIX.len()
  );
  for _ in 1..20 {
    let name = generate_star_name(&mut rng);

    print_var!(name);
  }
}
