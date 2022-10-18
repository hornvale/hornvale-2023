use rand::prelude::*;

use crate::astronomy::host_star::HostStar;
use crate::astronomy::terrestrial_planet::constants::*;
use crate::astronomy::terrestrial_planet::error::Error;
use crate::astronomy::terrestrial_planet::math::temperature::get_equilibrium_temperature;
use crate::astronomy::terrestrial_planet::rotation_direction::RotationDirection;
use crate::astronomy::terrestrial_planet::TerrestrialPlanet;

/// Constraints for creating a planet.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Constraints {
  /// The minimum mass.
  pub minimum_mass: Option<f64>,
  /// The maximum mass.
  pub maximum_mass: Option<f64>,
  /// The minimum axial tilt.
  pub minimum_axial_tilt: Option<f64>,
  /// The maximum axial tilt.
  pub maximum_axial_tilt: Option<f64>,
  /// The minimum rotational period.
  pub minimum_rotational_period: Option<f64>,
  /// The maximum rotational period.
  pub maximum_rotational_period: Option<f64>,
  /// The minimum orbital eccentricity.
  pub minimum_orbital_eccentricity: Option<f64>,
  /// The maximum orbital eccentricity.
  pub maximum_orbital_eccentricity: Option<f64>,
  /// The distance from the host star, in AU.
  pub host_star_distance: Option<f64>,
  /// The luminosity of the host star, in Lsol.
  pub host_star_luminosity: Option<f64>,
}

impl Constraints {
  /// No constraints, just let it all hang out.
  pub fn habitable() -> Self {
    let minimum_mass = Some(MINIMUM_HABITABLE_MASS);
    let maximum_mass = Some(MAXIMUM_HABITABLE_MASS);
    let minimum_rotational_period = Some(MINIMUM_HABITABLE_ROTATIONAL_PERIOD);
    let maximum_rotational_period = Some(MAXIMUM_HABITABLE_ROTATIONAL_PERIOD);
    let minimum_orbital_eccentricity = Some(MINIMUM_HABITABLE_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_HABITABLE_ORBITAL_ECCENTRICITY);
    Self {
      minimum_mass,
      maximum_mass,
      minimum_rotational_period,
      maximum_rotational_period,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      ..Constraints::default()
    }
  }

  /// Generate.
  #[named]
  pub fn generate<R: Rng + ?Sized>(
    &self,
    rng: &mut R,
    host_star: &HostStar,
    distance: f64,
  ) -> Result<TerrestrialPlanet, Error> {
    trace_enter!();
    let minimum_mass = self.minimum_mass.unwrap_or(MINIMUM_MASS);
    trace_var!(minimum_mass);
    let maximum_mass = self.maximum_mass.unwrap_or(MAXIMUM_MASS);
    trace_var!(maximum_mass);
    let mass = rng.gen_range(minimum_mass..maximum_mass);
    trace_var!(mass);
    let mut result = TerrestrialPlanet::from_mass(mass)?;
    trace_var!(result);
    let minimum_axial_tilt = self.minimum_axial_tilt.unwrap_or(0.0);
    trace_var!(minimum_axial_tilt);
    let maximum_axial_tilt = self.maximum_axial_tilt.unwrap_or(180.0);
    trace_var!(maximum_axial_tilt);
    let axial_tilt = rng.gen_range(minimum_axial_tilt..maximum_axial_tilt);
    trace_var!(axial_tilt);
    result.semi_major_axis = distance;
    result.axial_tilt = axial_tilt;
    result.rotation_direction = match axial_tilt {
      axial_tilt if axial_tilt > 0.0 && axial_tilt < 90.0 => RotationDirection::Prograde,
      axial_tilt if axial_tilt > 90.0 && axial_tilt < 180.0 => RotationDirection::Retrograde,
      _ => RotationDirection::Undefined,
    };
    result.tropic_zones = match axial_tilt {
      axial_tilt if axial_tilt < 90.0 => (0.0, axial_tilt),
      axial_tilt if axial_tilt > 90.0 => (0.0, 180.0 - axial_tilt),
      _ => (0.0, 0.0),
    };
    result.polar_zones = match axial_tilt {
      axial_tilt if axial_tilt < 90.0 => (90.0 - axial_tilt, 90.0),
      axial_tilt if axial_tilt > 90.0 => (90.0 - (180.0 - axial_tilt), 90.0),
      _ => (0.0, 0.0),
    };
    let minimum_orbital_eccentricity = self
      .minimum_orbital_eccentricity
      .unwrap_or(MINIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(minimum_orbital_eccentricity);
    let maximum_orbital_eccentricity = self
      .maximum_orbital_eccentricity
      .unwrap_or(MAXIMUM_ORBITAL_ECCENTRICITY);
    trace_var!(maximum_orbital_eccentricity);
    let orbital_eccentricity = rng.gen_range(minimum_orbital_eccentricity..maximum_orbital_eccentricity);
    result.orbital_eccentricity = orbital_eccentricity;
    trace_var!(orbital_eccentricity);
    let perihelion = (1.0 - orbital_eccentricity) * distance;
    result.perihelion = perihelion;
    trace_var!(perihelion);
    let aphelion = (1.0 + orbital_eccentricity) * distance;
    result.aphelion = aphelion;
    trace_var!(aphelion);
    let orbital_period = (distance.powf(3.0) / host_star.get_stellar_mass()).sqrt();
    result.orbital_period = orbital_period;
    trace_var!(orbital_period);
    let bond_albedo = result.bond_albedo;
    trace_var!(bond_albedo);
    let greenhouse_effect = result.greenhouse_effect;
    result.greenhouse_effect = greenhouse_effect;
    trace_var!(greenhouse_effect);
    let luminosity = host_star.get_luminosity();
    result.equilibrium_temperature = get_equilibrium_temperature(bond_albedo, greenhouse_effect, luminosity, distance);
    trace_exit!();
    Ok(result)
  }
}

impl Default for Constraints {
  /// No constraints, just let it all hang out.
  fn default() -> Self {
    let minimum_mass = None;
    let maximum_mass = None;
    let minimum_axial_tilt = None;
    let maximum_axial_tilt = None;
    let minimum_rotational_period = None;
    let maximum_rotational_period = None;
    let host_star_distance = None;
    let host_star_luminosity = None;
    let minimum_orbital_eccentricity = Some(MINIMUM_ORBITAL_ECCENTRICITY);
    let maximum_orbital_eccentricity = Some(MAXIMUM_ORBITAL_ECCENTRICITY);
    Self {
      minimum_mass,
      maximum_mass,
      minimum_axial_tilt,
      maximum_axial_tilt,
      minimum_rotational_period,
      maximum_rotational_period,
      minimum_orbital_eccentricity,
      maximum_orbital_eccentricity,
      host_star_distance,
      host_star_luminosity,
    }
  }
}

#[cfg(test)]
pub mod test {

  use crate::astronomy::host_star::constraints::Constraints as HostStarConstraints;
  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_generate() -> Result<(), Error> {
    init();
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let host_star_constraints = HostStarConstraints::habitable();
    let mut host_star = host_star_constraints.generate(&mut rng)?;
    trace_var!(host_star);
    let mut is_habitable = !host_star.is_habitable();
    let mut counter = 0;
    while !is_habitable && counter < 50 {
      host_star = host_star_constraints.generate(&mut rng)?;
      is_habitable = !host_star.is_habitable();
      counter += 1;
    }
    let habitable_zone = host_star.get_habitable_zone();
    trace_var!(habitable_zone);
    let distance = rng.gen_range(habitable_zone.0..habitable_zone.1);
    trace_var!(distance);
    let planet = &Constraints::default().generate(&mut rng, &host_star, distance)?;
    trace_var!(planet);
    print_var!(planet);
    trace_exit!();
    Ok(())
  }
}
