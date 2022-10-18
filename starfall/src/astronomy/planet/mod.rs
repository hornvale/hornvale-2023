use crate::astronomy::gas_giant_planet::GasGiantPlanet;
use crate::astronomy::terrestrial_planet::TerrestrialPlanet;

pub mod constants;
pub mod constraints;
pub mod error;
use error::Error;
pub mod math;

/// The `Planet` class.  This will get complicated.
#[derive(Clone, Debug, PartialEq)]
pub enum Planet {
  /// Gas Giant Planet.
  GasGiantPlanet(GasGiantPlanet),
  /// Terrestrial Planet.
  TerrestrialPlanet(TerrestrialPlanet),
}

impl Planet {
  /// Get density of the planet.
  #[named]
  pub fn get_density(&self) -> f64 {
    trace_enter!();
    use Planet::*;
    let result = match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.density,
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.density,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get mass of the planet.
  #[named]
  pub fn get_mass(&self) -> f64 {
    trace_enter!();
    use Planet::*;
    let result = match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.mass,
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.mass,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get radius of the planet.
  #[named]
  pub fn get_radius(&self) -> f64 {
    trace_enter!();
    use Planet::*;
    let result = match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.radius,
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.radius,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Get the orbital period of the planet.
  #[named]
  pub fn get_orbital_period(&self) -> f64 {
    trace_enter!();
    use Planet::*;
    let result = match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.orbital_period,
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.orbital_period,
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  #[named]
  pub fn check_habitable(&self) -> Result<(), Error> {
    trace_enter!();
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.check_habitable()?,
      _ => return Err(Error::UninhabitablePlanetType),
    }
    let result = Ok(());
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  #[named]
  pub fn is_habitable(&self) -> bool {
    trace_enter!();
    let result = match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
