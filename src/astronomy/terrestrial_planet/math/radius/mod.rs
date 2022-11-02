/// Calculate the radius of a terrestrial planet, given its mass and density.
///
/// Units are Mearth, Dearth, and Rearth.
#[named]
pub fn get_radius(mass: f64, density: f64) -> f64 {
  (mass / (density / 5.51)).powf(1.0 / 3.0)
}
