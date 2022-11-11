/// This is an approximation of the innermost sustainable orbit of a satellite
/// around a star of some kind.
///
/// * `mass` - Host body mass in Msol.
///
/// Returns distance in AU.
pub fn get_approximate_innermost_orbit(mass: f64) -> f64 {
  0.01 * mass
}

/// This is an approximation of the outermost sustainable orbit of a satellite
/// around a host body of some kind.
///
/// * `mass` - Host body mass in Msol.
///
/// Returns distance in AU.
pub fn get_approximate_outermost_orbit(mass: f64) -> f64 {
  40.0 * mass
}
