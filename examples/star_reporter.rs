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

  pub fn report_string(&self, indent: usize, string: &str) {
    println!("{:indent$}{}", "", string, indent = indent);
  }

  pub fn report(&self, star: &Star, indent: usize) {
    let new_indent = indent + 2;
    self.report_string(new_indent, &format!("This is a {} class star.", star.class));
    self.report_string(
      new_indent,
      &format!("It is about {:.0} times the mass of the sun.", star.mass),
    );
    self.report_string(new_indent, "It burns.");
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
      self.report_string(new_indent, "It is habitable.");
    } else {
      match star.check_habitable() {
        Err(error) => {
          self.report_string(new_indent, &format!("It is not habitable; {}.", error));
        },
        Ok(_) => unreachable!(),
      }
    }
    print_var!(star);
  }
}

fn main() -> Result<(), Error> {
  init_pretty_env_logger();
  let mut rng = rand::thread_rng();
  let constraints = Constraints::default();
  let star = constraints.generate(&mut rng)?;
  let reporter = StarReporter::new();
  reporter.report(&star, 2);

  Ok(())
}
