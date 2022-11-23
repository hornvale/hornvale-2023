use crate::astronomy::_type::*;

/// Calculate the escape velocity of a terrestrial planet.
///
/// Units are Mearth, Rearth, and Vearth.
pub fn get_escape_velocity(mass: MEarth, radius: REarth) -> f64 {
  (mass.0 / radius.0).sqrt()
}
