/// Calculate the radius of a terrestrial planet, given its mass and density.
///
/// Units are Mearth, Dearth, and Rearth.
#[named]
pub fn get_radius(mass: f64, density: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  trace_var!(density);
  let result = (mass / (density / 5.51)).powf(1.0 / 3.0);
  trace_var!(result);
  trace_exit!();
  result
}
