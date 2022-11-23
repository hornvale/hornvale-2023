use crate::astronomy::_type::*;
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
  /// The mass of this moon, in MLuna.
  pub mass: MLuna,
  /// The density of this moon, in DLuna.
  pub density: DLuna,
  /// The radius of this moon, in RLuna.
  pub radius: RLuna,
  /// The gravity of this moon, in Gearth (not GLuna).
  pub gravity: GEarth,
  /// The escape velocity of this moon, in KM/sec.
  pub escape_velocity: VKmSec,
  /// The Bond albedo of this moon (unitless).
  pub bond_albedo: f64,
  /// Semi-major axis, in KM.
  pub semi_major_axis: LKm,
  /// Orbital eccentricity (unitless).
  pub orbital_eccentricity: f64,
  /// Periapsis, in KM.
  pub periapsis: LKm,
  /// Apoapsis, in KM.
  pub apoapsis: LKm,
  /// Orbital inclination.
  pub orbital_inclination: f64,
  /// Rotation direction.
  pub rotation_direction: RotationDirection,
  /// Sidereal orbital period, in TEarthDay.
  pub sidereal_orbital_period: TEarthDay,
  /// Normal orbital period, in TEarthDay.
  pub orbital_period: TEarthDay,
  /// Rotational period, in TEarthHour.
  pub rotation_period: TEarthDay,
  /// Lunar tide, in meters.
  pub lunar_tide: Lm,
  /// Solar tide, in meters.
  pub solar_tide: Lm,
  /// Planetary tide, in meters.
  pub planetary_tide: Lm,
  /// Spring tides, in meters.
  pub spring_tide_magnitude: Lm,
  /// Neap tide magnitude, in meters.
  pub neap_tide_magnitude: Lm,
  /// If the planet is tidally locked to this moon.
  pub is_planet_tidally_locked: bool,
  /// If the moon is tidally locked to the planet.
  pub is_moon_tidally_locked: bool,
}

impl Moon {
  pub fn from_environment(
    mass: MLuna,
    host_star: &HostStar,
    star_distance: LAu,
    planet: &Planet,
    planet_distance: LKm,
  ) -> Result<Moon, Error> {
    let density = DLuna(1.0);
    let _d_gm_cm3 = density.0 * 3.34;
    let radius = RLuna((mass.0 / density.0).powf(1.0 / 3.0));
    // This gives gravity in Earth equivalents, since other units are relative
    // to the Moon, and GLuna is 0.1654 * Gearth.
    let g_luna = GLuna(mass.0 / radius.0.powf(2.0));
    let gravity = GEarth(g_luna.0 * 0.1654);
    // This is in KM/sec.
    let escape_velocity = VKmSec((mass.0 / radius.0).sqrt() * 2.380);
    // Peg this to the albedo of the Moon for the time being.
    let bond_albedo = 0.136;
    let semi_major_axis = planet_distance;
    // Pegged for the time being.
    let orbital_eccentricity = 0.05;
    let periapsis = LKm((1.0 - orbital_eccentricity) * semi_major_axis.0);
    let apoapsis = LKm((1.0 + orbital_eccentricity) * semi_major_axis.0);
    // Pegged.
    let orbital_inclination = 5.15;
    let rotation_direction = RotationDirection::Prograde;
    let mass_coefficient = planet.get_mass().0 + <m_luna::MLuna as Into<MKg>>::into(mass).0 * 0.0123;
    let sidereal_orbital_period =
      TEarthDay(0.0588 * ((semi_major_axis.0 / 12_742.0 * 2.0).powf(3.0) / mass_coefficient).sqrt());
    let earth_orbital_period = planet.get_orbital_period() * 365.265;
    let orbital_period = TEarthDay(earth_orbital_period.0 / (earth_orbital_period.0 / sidereal_orbital_period.0 - 1.0));
    let lunar_tide = get_lunar_tide(mass, planet.get_radius().into(), semi_major_axis);
    let solar_tide = get_solar_tide(host_star.get_stellar_mass(), planet.get_radius().into(), star_distance);
    let planetary_tide = get_planetary_tide(mass, radius, semi_major_axis);
    let spring_tide_magnitude = get_spring_tide(lunar_tide, solar_tide);
    let neap_tide_magnitude = get_neap_tide(lunar_tide, solar_tide);
    let is_planet_tidally_locked = is_planet_tidally_locked(
      lunar_tide,
      solar_tide,
      host_star.get_current_age(),
      planet.get_mass().into(),
    );
    let is_moon_tidally_locked = is_moon_tidally_locked(solar_tide, planetary_tide, host_star.get_current_age(), mass);
    let rotation_period = if is_moon_tidally_locked {
      orbital_period
    } else {
      TEarthDay(3.0)
    };
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
