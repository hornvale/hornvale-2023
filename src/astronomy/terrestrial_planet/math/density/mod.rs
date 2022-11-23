use crate::astronomy::_type::*;

/// Calculate the density of terrestrial planet, given its mass and CMF.
///
/// The CMF, or Core Mass Fraction, indicates what percentage of the planet's
/// mass is contained within its iron core.
///
/// Given that, we can calculate the overall density of the planet in DEarth.
pub fn get_density(mass: MEarth, cmf: f64) -> DEarth {
  let d1 = 5.51 * mass.0.powf(0.189) / (1.07 - 0.21 * (cmf)).powf(3.0);
  let d2 = 3.5 + 4.37 * cmf;
  let result = match mass.0 {
    mass if mass > 0.6 => d1,
    _mass if d1 > d2 => d1,
    _ => d2,
  };
  DGmCm3(result).into()
}
