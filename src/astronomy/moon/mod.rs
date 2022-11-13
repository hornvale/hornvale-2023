use crate::astronomy::host_star::HostStar;
use crate::astronomy::planet::Planet;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;
use math::tides::{
  get_lunar_tide, get_neap_tide, get_planetary_tide, get_solar_tide, get_spring_tide, is_moon_tidally_locked,
  is_planet_tidally_locked,
};
pub mod rotation_direction;
use rotation_direction::RotationDirection;

/// A `Moon`, mercifully, is a fairly simple concept.
///
/// It's possible that at some point, we might make moons habitable.
///
/// For instance, a habitable moon of a hot jupiter gas giant.
///
/// But for now, we're just staying with terrestrial planets, and we'll assume
/// that moons are just celestial features.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Moon {
  /// The mass of this moon, in Mmoon.
  pub mass: f64,
  /// The density of this moon, in Dmoon.
  pub density: f64,
  /// The radius of this moon, in Rmoon.
  pub radius: f64,
  /// The gravity of this moon, in Gearth (not Gmoon).
  pub gravity: f64,
  /// The escape velocity of this moon, in KM/sec.
  pub escape_velocity: f64,
  /// The Bond albedo of this moon.
  pub bond_albedo: f64,
  /// Semi-major axis, in KM.
  pub semi_major_axis: f64,
  /// Orbital eccentricity.
  pub orbital_eccentricity: f64,
  /// Periapsis.
  pub periapsis: f64,
  /// Apoapsis.
  pub apoapsis: f64,
  /// Orbital inclination.
  pub orbital_inclination: f64,
  /// Rotation direction.
  pub rotation_direction: RotationDirection,
  /// Sidereal orbital period.
  pub sidereal_orbital_period: f64,
  /// Normal orbital period.
  pub orbital_period: f64,
  /// Rotational period.
  pub rotation_period: f64,
  /// Lunar tide.
  pub lunar_tide: f64,
  /// Solar tide.
  pub solar_tide: f64,
  /// Planetary tide.
  pub planetary_tide: f64,
  /// Spring tides.
  pub spring_tide_magnitude: f64,
  /// Neap tide magnitude.
  pub neap_tide_magnitude: f64,
  /// If the planet is tidally locked to this moon.
  pub is_planet_tidally_locked: bool,
  /// If the moon is tidally locked to the planet.
  pub is_moon_tidally_locked: bool,
}

impl Moon {
  pub fn from_environment(
    mass: f64,
    host_star: &HostStar,
    star_distance: f64,
    planet: &Planet,
    planet_distance: f64,
  ) -> Result<Moon, Error> {
    let density = 3.34;
    let radius = (mass / (density / 3.34)).powf(1.0 / 3.0);
    // This gives gravity in Earth equivalents, since other units are relative
    // to the Moon, and Gmoon is 0.1654 * Gearth.
    let gravity = (mass / radius.powf(2.0)) * 0.1654;
    // This is in KM/sec.
    let escape_velocity = (mass / radius).sqrt() * 2.380;
    // Peg this to the albedo of the Moon for the time being.
    let bond_albedo = 0.136;
    let semi_major_axis = planet_distance;
    // Pegged for the time being.
    let orbital_eccentricity = 0.05;
    let periapsis = (1.0 - orbital_eccentricity) * semi_major_axis;
    let apoapsis = (1.0 + orbital_eccentricity) * semi_major_axis;
    // Pegged.
    let orbital_inclination = 5.15;
    let rotation_direction = RotationDirection::Prograde;
    let sidereal_orbital_period =
      0.0588 * ((semi_major_axis / 12_742.0 * 2.0).powf(3.0) / (planet.get_mass() + mass * 0.0123)).sqrt();
    let earth_orbital_period = planet.get_orbital_period() * 365.265;
    let orbital_period = earth_orbital_period / (earth_orbital_period / sidereal_orbital_period - 1.0);
    let lunar_tide = get_lunar_tide(mass, planet.get_radius(), semi_major_axis);
    let solar_tide = get_solar_tide(host_star.get_stellar_mass(), planet.get_radius(), star_distance);
    let planetary_tide = get_planetary_tide(mass, radius, semi_major_axis);
    let spring_tide_magnitude = get_spring_tide(lunar_tide, solar_tide);
    let neap_tide_magnitude = get_neap_tide(lunar_tide, solar_tide);
    let is_planet_tidally_locked =
      is_planet_tidally_locked(lunar_tide, solar_tide, host_star.get_current_age(), planet.get_mass());
    let is_moon_tidally_locked = is_moon_tidally_locked(solar_tide, planetary_tide, host_star.get_current_age(), mass);
    let rotation_period = if is_moon_tidally_locked { orbital_period } else { 3.0 };
    let result = Moon {
      mass,
      density,
      radius,
      gravity,
      escape_velocity,
      bond_albedo,
      semi_major_axis,
      orbital_eccentricity,
      periapsis,
      apoapsis,
      orbital_inclination,
      rotation_direction,
      sidereal_orbital_period,
      orbital_period,
      rotation_period,
      lunar_tide,
      solar_tide,
      planetary_tide,
      spring_tide_magnitude,
      neap_tide_magnitude,
      is_planet_tidally_locked,
      is_moon_tidally_locked,
    };
    Ok(result)
  }
}
