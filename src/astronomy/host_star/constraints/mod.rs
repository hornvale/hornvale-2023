use rand::prelude::*;
use std::default::Default;

use crate::astronomy::close_binary_star::constraints::Constraints as CloseBinaryStarConstraints;
use crate::astronomy::host_star::constants::*;
use crate::astronomy::host_star::error::Error;
use crate::astronomy::host_star::HostStar;
use crate::astronomy::star::constraints::Constraints as StarConstraints;

/// Constraints for creating a main-sequence host star.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// Star constraints.
  pub star_constraints: Option<StarConstraints>,
  /// Close Binary Star constraints.
  pub close_binary_star_constraints: Option<CloseBinaryStarConstraints>,
}

impl Constraints {
  /// Generate a habitable host star.

  pub fn habitable() -> Self {
    let star_constraints = Some(StarConstraints::habitable());
    let close_binary_star_constraints = Some(CloseBinaryStarConstraints::habitable());

    Self {
      star_constraints,
      close_binary_star_constraints,
    }
  }

  /// Generate.

  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<HostStar, Error> {
    use HostStar::*;
    let is_solitary: bool = rng.gen_range(0.0..=1.0) > BINARY_STAR_PROBABILITY;
    let result = if is_solitary {
      let constraints = self.star_constraints.unwrap_or_default();
      Star(Box::new(constraints.generate(rng)?))
    } else {
      let constraints = self.close_binary_star_constraints.unwrap_or_default();
      CloseBinaryStar(Box::new(constraints.generate(rng)?))
    };

    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let star_constraints = None;
    let close_binary_star_constraints = None;
    Self {
      star_constraints,
      close_binary_star_constraints,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let host_star = Constraints::default().generate(&mut rng)?;

    print_var!(host_star);

    Ok(())
  }

  #[test]
  pub fn test_random() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let mut binary_count = 0;
    for _ in 1..10 {
      if let Ok(host_star) = Constraints::default().generate(&mut rng) {
        if let HostStar::CloseBinaryStar(_) = host_star {
          binary_count += 1;
        }
      }
    }
    print_var!(binary_count);

    Ok(())
  }

  #[test]
  pub fn find_habitable() -> Result<(), Error> {
    init();
    let mut rng = thread_rng();
    let mut habitable_count = 0;
    for _ in 1..1000 {
      if let Ok(host_star) = Constraints::habitable().generate(&mut rng) {
        if host_star.is_habitable() {
          habitable_count += 1;
        } else {
          print_var!(host_star);
          if let Err(error) = host_star.check_habitable() {
            print_var!(error);
          }
        }
      }
    }
    print_var!(habitable_count);

    Ok(())
  }
}
