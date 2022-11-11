/// Calculate the escape velocity of a terrestrial planet.
///
/// Units are Mearth, Rearth, and Vearth.
pub fn get_escape_velocity(mass: f64, radius: f64) -> f64 {
  (mass / radius).sqrt()
}
