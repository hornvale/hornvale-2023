/// Calculate the gravity of a terrestrial planet, given its mass and radius.
///
/// Units are Mearth, Rearth, and Gearth.
pub fn get_gravity(mass: f64, radius: f64) -> f64 {
  mass / radius.powf(2.0)
}
