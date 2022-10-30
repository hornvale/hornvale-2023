pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;

/// The `GasGiantPlanet` type.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct GasGiantPlanet {
  /// Mass, in Mjupiter.
  pub mass: f64,
  /// Density, in Djupiter.
  pub density: f64,
  /// Radius, in Rjupiter.
  pub radius: f64,
  /// Semi-Major Axis.
  pub semi_major_axis: f64,
  /// Orbital eccentricity.
  pub orbital_eccentricity: f64,
  /// Perihelion.
  pub perihelion: f64,
  /// Aphelion.
  pub aphelion: f64,
  /// Orbital period, in Earth years.
  pub orbital_period: f64,
}

impl GasGiantPlanet {
  #[named]
  pub fn from_mass(mass: f64) -> Result<Self, Error> {
    trace_enter!();
    trace_var!(mass);
    // @todo: fix.
    let density: f64 = 1.0;
    trace_var!(density);
    // @todo: fix.
    let radius: f64 = 1.0;
    trace_var!(radius);
    let semi_major_axis: f64 = 5.2;
    trace_var!(semi_major_axis);
    let orbital_eccentricity = 0.0167;
    trace_var!(orbital_eccentricity);
    let perihelion = (1.0 - orbital_eccentricity) * semi_major_axis;
    trace_var!(perihelion);
    let aphelion = (1.0 + orbital_eccentricity) * semi_major_axis;
    trace_var!(aphelion);
    let orbital_period = semi_major_axis.powf(3.0).sqrt();
    trace_var!(orbital_period);
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
    trace_var!(result);
    trace_exit!();
    Ok(result)
  }
}
