use crate::astronomy::_type::*;
use crate::astronomy::gas_giant_planet::GasGiantPlanet;
use crate::astronomy::terrestrial_planet::TerrestrialPlanet;
pub mod constraints;
pub mod error;
use error::Error;

/// The `Planet` class.  This will get complicated.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Planet {
  /// Gas Giant Planet.
  GasGiantPlanet(GasGiantPlanet),
  /// Terrestrial Planet.
  TerrestrialPlanet(TerrestrialPlanet),
}

impl Planet {
  /// Get density of the planet.
  pub fn get_density(&self) -> DGmCm3 {
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.density.into(),
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.density.into(),
    }
  }

  /// Get mass of the planet.
  pub fn get_mass(&self) -> MKg {
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.mass.into(),
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.mass.into(),
    }
  }

  /// Get radius of the planet.
  pub fn get_radius(&self) -> LKm {
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.radius.into(),
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.radius.into(),
    }
  }

  /// Get the orbital period of the planet.
  pub fn get_orbital_period(&self) -> TEarthYear {
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.orbital_period,
      GasGiantPlanet(gas_giant_planet) => gas_giant_planet.orbital_period,
    }
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  pub fn check_habitable(&self) -> Result<(), Error> {
    use Planet::*;
    match &self {
      TerrestrialPlanet(terrestrial_planet) => terrestrial_planet.check_habitable()?,
      _ => return Err(Error::UninhabitablePlanetType),
    }
    Ok(())
  }

  /// Indicate whether this planet is capable of supporting conventional life.
  pub fn is_habitable(&self) -> bool {
    match self.check_habitable() {
      Ok(()) => true,
      Err(_) => false,
    }
  }
}
