pub mod constants;
use constants::*;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;
use math::atmospheric_stability::*;
use math::density::get_density;
use math::escape_velocity::get_escape_velocity;
use math::gravity::get_gravity;
use math::radius::get_radius;
use math::temperature::get_equilibrium_temperature;
pub mod rotation_direction;
use rotation_direction::RotationDirection;

/// The `TerrestrialPlanet` type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TerrestrialPlanet {
  /// Mass in Mearth.
  pub mass: f64,
  /// Core Mass Fraction.
  pub core_mass_fraction: f64,
  /// Density, in Dearth.
  pub density: f64,
  /// Escape velocity, in Vearth.
  pub escape_velocity: f64,
  /// Gravity, in Gearth.
  pub gravity: f64,
  /// Radius, in Rearth.
  pub radius: f64,
  /// Axial tilt (0-180º).
  pub axial_tilt: f64,
  /// Rotation.
  pub rotation_direction: RotationDirection,
  /// Semi-Major Axis.
  pub semi_major_axis: f64,
  /// Tropic Zone.
  pub tropic_zones: (f64, f64),
  /// Polar Zones.
  pub polar_zones: (f64, f64),
  /// Orbital eccentricity.
  pub orbital_eccentricity: f64,
  /// Perihelion.
  pub perihelion: f64,
  /// Aphelion.
  pub aphelion: f64,
  /// Orbital period, in Earth years.
  pub orbital_period: f64,
  /// Bond albedo.
  pub bond_albedo: f64,
  /// Greenhouse effect.
  pub greenhouse_effect: f64,
  /// Equilibrium temperature, in Kelvin.
  pub equilibrium_temperature: f64,
  /// Whether we can retain the gases necessary for conventional life.
  pub is_atmospherically_stable: bool,
}

impl TerrestrialPlanet {
  pub fn from_mass(mass: f64) -> Result<Self, Error> {
    let core_mass_fraction: f64 = 0.35;
    let density = get_density(mass, core_mass_fraction);
    let radius = get_radius(mass, density);
    let escape_velocity = get_escape_velocity(mass, radius);
    let gravity = get_gravity(mass, radius);
    let axial_tilt = 23.5;
    let rotation_direction = RotationDirection::Prograde;
    let tropic_zones = (0.0, axial_tilt);
    let polar_zones = (90.0 - axial_tilt, 90.0);
    let bond_albedo = 0.29;
    let greenhouse_effect = 1.0;
    let host_star_luminosity = 1.0;
    let host_star_distance = 1.0;
    let semi_major_axis: f64 = host_star_distance;
    let orbital_eccentricity = 0.0167;
    let perihelion = (1.0 - orbital_eccentricity) * semi_major_axis;
    let aphelion = (1.0 + orbital_eccentricity) * semi_major_axis;
    let orbital_period = semi_major_axis.powf(3.0).sqrt();
    let equilibrium_temperature =
      get_equilibrium_temperature(bond_albedo, greenhouse_effect, host_star_luminosity, host_star_distance);
    let is_atmospherically_stable = is_atmospherically_stable(equilibrium_temperature, escape_velocity);
    let result = Self {
      mass,
      core_mass_fraction,
      density,
      escape_velocity,
      gravity,
      radius,
      axial_tilt,
      rotation_direction,
      semi_major_axis,
      tropic_zones,
      polar_zones,
      orbital_eccentricity,
      perihelion,
      aphelion,
      orbital_period,
      bond_albedo,
      greenhouse_effect,
      equilibrium_temperature,
      is_atmospherically_stable,
    };
    Ok(result)
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
    {
      if self.equilibrium_temperature <= MINIMUM_HABITABLE_TEMPERATURE {
        // About 0ºC is too damned cold.
        return Err(Error::TooColdToSupportConventionalLife);
      }
      if self.equilibrium_temperature >= MAXIMUM_HABITABLE_TEMPERATURE {
        // About 50ºC is too damned hot.
        return Err(Error::TooHotToSupportConventionalLife);
      }
      if self.gravity <= MINIMUM_HABITABLE_GRAVITY {
        return Err(Error::GravityTooLowToSupportConventionalLife);
      }
      if self.gravity >= MAXIMUM_HABITABLE_GRAVITY {
        return Err(Error::GravityTooHighToSupportConventionalLife);
      }
      if !is_carbon_dioxide_stable(self.equilibrium_temperature, self.escape_velocity) {
        return Err(Error::AtmosphereUnstableForCarbonDioxide);
      }
      if !is_argon_stable(self.equilibrium_temperature, self.escape_velocity) {
        return Err(Error::AtmosphereUnstableForArgon);
      }
      if !is_oxygen_stable(self.equilibrium_temperature, self.escape_velocity) {
        return Err(Error::AtmosphereUnstableForOxygen);
      }
      if !is_nitrogen_stable(self.equilibrium_temperature, self.escape_velocity) {
        return Err(Error::AtmosphereUnstableForNitrogen);
      }
      Ok(())
    }
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_from_mass() -> Result<(), Error> {
    init();
    let planet = TerrestrialPlanet::from_mass(1.0)?;
    assert_approx_eq!(planet.mass, 1.0);
    assert_approx_eq!(planet.core_mass_fraction, 0.35);
    assert_approx_eq!(planet.density, 5.56, 0.01);
    assert_approx_eq!(planet.escape_velocity, 1.00, 0.01);
    assert_approx_eq!(planet.gravity, 1.00, 0.01);
    assert_approx_eq!(planet.radius, 1.00, 0.01);

    print_var!(planet);
    Ok(())
  }
}
