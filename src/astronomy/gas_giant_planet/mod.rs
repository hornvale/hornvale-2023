pub mod constants;
pub mod constraints;
pub mod error;
use crate::astronomy::_type::*;
use error::Error;

/// The `GasGiantPlanet` type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GasGiantPlanet {
  /// Mass, in Mjupiter.
  pub mass: MJupiter,
  /// Density, in Djupiter.
  pub density: DJupiter,
  /// Radius, in Rjupiter.
  pub radius: RJupiter,
  /// Semi-Major Axis.
  pub semi_major_axis: LAu,
  /// Orbital eccentricity (unitless).
  pub orbital_eccentricity: f64,
  /// Perihelion.
  pub perihelion: LAu,
  /// Aphelion.
  pub aphelion: LAu,
  /// Orbital period, in Earth years.
  pub orbital_period: TEarthYear,
}

impl GasGiantPlanet {
  pub fn from_mass(mass: MJupiter) -> Result<Self, Error> {
    // @todo: fix.
    let density: DJupiter = DJupiter(1.0);
    // @todo: fix.
    let radius: RJupiter = RJupiter(1.0);
    let semi_major_axis: LAu = LAu(5.2);
    let orbital_eccentricity = 0.0167;
    let perihelion = LAu((1.0 - orbital_eccentricity) * semi_major_axis.0);
    let aphelion = LAu((1.0 + orbital_eccentricity) * semi_major_axis.0);
    let orbital_period = TEarthYear(semi_major_axis.0.powf(3.0).sqrt());
    let result = Self {
      mass,
      density,
      radius,
      semi_major_axis,
      orbital_eccentricity,
      perihelion,
      aphelion,
      orbital_period,
    };
    Ok(result)
  }
}
