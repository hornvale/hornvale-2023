use crate::astronomy::_type::*;

/// This is an approximation of the innermost sustainable orbit of a satellite
/// around a star of some kind.
///
/// * `mass` - Host body mass in Msol.
///
/// Returns distance in AU.
pub fn get_approximate_innermost_orbit(mass: MSol) -> LAu {
  LAu(0.01 * mass.0)
}

/// This is an approximation of the outermost sustainable orbit of a satellite
/// around a host body of some kind.
///
/// * `mass` - Host body mass in Msol.
///
/// Returns distance in AU.
pub fn get_approximate_outermost_orbit(mass: MSol) -> LAu {
  LAu(40.0 * mass.0)
}
