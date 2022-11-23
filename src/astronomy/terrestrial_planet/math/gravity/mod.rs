use crate::astronomy::_type::*;

/// Calculate the gravity of a terrestrial planet, given its mass and radius.
///
/// Units are Mearth, Rearth, and Gearth.
pub fn get_gravity(mass: MEarth, radius: REarth) -> f64 {
  mass.0 / radius.0.powf(2.0)
}
