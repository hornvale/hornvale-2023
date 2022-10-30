#![allow(unused_imports)]
///! Generates a star system and prints a little report on it.
use hornvale::astronomy::star::constraints::Constraints;
use hornvale::astronomy::star::error::Error;
use hornvale::astronomy::star::Star;
use hornvale::*;
use rand::prelude::*;

#[macro_use]
extern crate function_name;

pub struct StarReporter {}

impl StarReporter {
  pub fn new() -> Self {
    Self {}
  }

  #[named]
  pub fn report_string(&self, indent: usize, string: &str) {
    trace_enter!();
    println!("{:indent$}{}", "", string, indent = indent);
    trace_exit!();
  }

  #[named]
  pub fn report(&self, star: &Star, indent: usize) {
    trace_enter!();
    let new_indent = indent + 2;
    self.report_string(new_indent, &format!("This is a {} class star.", star.class));
    self.report_string(
      new_indent,
      &format!("It is about {:.0} times the mass of the sun.", star.mass),
    );
    self.report_string(new_indent, &format!("It burns."));
    let absolute_rgb = star.absolute_rgb;
    self.report_string(
      new_indent,
      &format!(
        "Its color is #{:02X}{:02X}{:02X}.",
        absolute_rgb.0, absolute_rgb.1, absolute_rgb.2
      ),
    );
    let is_habitable = star.is_habitable();
    if is_habitable {
      self.report_string(new_indent, &format!("It is habitable."));
    } else {
      match star.check_habitable() {
        Err(error) => {
          self.report_string(new_indent, &format!("It is not habitable; {}.", error));
        },
        Ok(_) => unreachable!(),
      }
    }
    print_var!(star);
    trace_exit!();
  }
}

#[named]
fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  trace_enter!();
  let mut rng = rand::thread_rng();
  let constraints = Constraints::default();
  let star = constraints.generate(&mut rng)?;
  let reporter = StarReporter::new();
  reporter.report(&star, 2);
  trace_exit!();
  Ok(())
}
