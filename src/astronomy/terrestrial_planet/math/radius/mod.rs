use crate::astronomy::_type::*;

/// Calculate the radius of a terrestrial planet, given its mass and density.
///
/// Units are Mearth, Dearth, and Rearth.
pub fn get_radius(mass: MEarth, density: DEarth) -> REarth {
  REarth((mass.0 / (density.0 / 5.51)).powf(1.0 / 3.0))
}
