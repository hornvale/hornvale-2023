use rand::prelude::*;
use std::default::Default;

use crate::astronomy::host_star::HostStar;
use crate::astronomy::satellite_system::constraints::Constraints as SatelliteSystemConstraints;
use crate::astronomy::satellite_systems::constants::*;
use crate::astronomy::satellite_systems::error::Error;
use crate::astronomy::satellite_systems::SatelliteSystems;

/// Constraints for creating satellite systems.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum number to generate.
  pub minimum_count: Option<usize>,
  /// The maximum number to generate.
  pub maximum_count: Option<usize>,
  /// Satellite System constraints.
  pub satellite_system_constraints: Option<SatelliteSystemConstraints>,
  /// Generate a primary gas giant.
  pub generate_primary_gas_giant: bool,
  /// Generate a habitable planet.
  pub generate_habitable: bool,
}

impl Constraints {
  /// Generate a habitable star subsystem.
  #[named]
  pub fn habitable() -> Self {
    trace_enter!();
    let generate_primary_gas_giant = true;
    let generate_habitable = true;
    let satellite_system_constraints = Some(SatelliteSystemConstraints::habitable());
    let result = Self {
      generate_primary_gas_giant,
      generate_habitable,
      satellite_system_constraints,
      ..Constraints::default()
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(&self, rng: &mut R, host_star: &HostStar) -> Result<SatelliteSystems, Error> {
    trace_enter!();
    let minimum_count = self.minimum_count.unwrap_or(MINIMUM_SATELLITE_SYSTEMS);
    trace_var!(minimum_count);
    let maximum_count = self.maximum_count.unwrap_or(MAXIMUM_SATELLITE_SYSTEMS);
    trace_var!(maximum_count);
    let satellite_system_constraints = self.satellite_system_constraints.unwrap_or_default();
    trace_var!(satellite_system_constraints);
    let mut satellite_systems = Vec::new();
    let orbits = self.generate_orbits(rng, host_star)?;
    for orbit in orbits.into_iter() {
      let satellite_system = satellite_system_constraints.generate(rng, host_star, orbit)?;
      trace_var!(satellite_system);
      satellite_systems.push(satellite_system);
    }
    trace_var!(satellite_systems);
    let result = SatelliteSystems { satellite_systems };
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }

  /// Generate orbits.
  #[named]
  pub fn generate_orbits<R: Rng + ?Sized>(&self, rng: &mut R, host_star: &HostStar) -> Result<Vec<f64>, Error> {
    trace_enter!();
    trace_var!(host_star);
    let mut result = Vec::new();
    if self.generate_primary_gas_giant {
      let orbit = rng.gen_range(1.0..1.25) + host_star.get_frost_line();
      result.push(orbit);
    }
    if self.generate_habitable {
      let habitable_zone = host_star.get_habitable_zone();
      let orbit = rng.gen_range(habitable_zone.0..habitable_zone.1);
      result.push(orbit);
    }
    let satellite_zone = host_star.get_satellite_zone();
    trace_var!(satellite_zone);
    let innermost_orbit = satellite_zone.0;
    trace_var!(innermost_orbit);
    let outermost_orbit = satellite_zone.1;
    trace_var!(outermost_orbit);
    let minimum = 40.0 * innermost_orbit;
    trace_var!(minimum);
    let distance_limit = outermost_orbit;
    trace_var!(distance_limit);
    let growth_factor = 0.3;
    trace_var!(growth_factor);
    let mut orbital_distance = minimum;
    let mut index = 0;
    loop {
      let min_unwrapped = 0.80 * orbital_distance;
      let max_unwrapped = 1.25 * orbital_distance;
      if !result
        .iter()
        .any(|&orbit| orbit > min_unwrapped && orbit < max_unwrapped)
      {
        let orbit = rng.gen_range(min_unwrapped..max_unwrapped);
        result.push(orbit);
      }
      orbital_distance = minimum + growth_factor * (2.0_f64).powf(index as f64);
      index += 1;
      if orbital_distance > distance_limit {
        break;
      }
    }
    result.sort_by(|a, b| a.partial_cmp(b).unwrap());
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_count = None;
    let maximum_count = None;
    let satellite_system_constraints = None;
    let generate_primary_gas_giant = false;
    let generate_habitable = false;
    Self {
      minimum_count,
      maximum_count,
      satellite_system_constraints,
      generate_primary_gas_giant,
      generate_habitable,
    }
  }
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let host_star = &HostStarConstraints::default().generate(&mut rng)?;
    trace_var!(host_star);
    let satellite_systems = &Constraints::default().generate(&mut rng, host_star)?;
    trace_var!(satellite_systems);
    print_var!(satellite_systems);
    trace_exit!();
    Ok(())
  }

  #[named]
  #[test]
  pub fn test_habitable() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let host_star = &HostStarConstraints::habitable().generate(&mut rng)?;
    trace_var!(host_star);
    let satellite_systems = &Constraints::habitable().generate(&mut rng, host_star)?;
    trace_var!(satellite_systems);
    print_var!(satellite_systems);
    trace_exit!();
    Ok(())
  }
}
