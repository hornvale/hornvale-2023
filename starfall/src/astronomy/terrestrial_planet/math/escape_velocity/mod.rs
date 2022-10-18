/// Calculate the escape velocity of a terrestrial planet.
///
/// Units are Mearth, Rearth, and Vearth.
#[named]
pub fn get_escape_velocity(mass: f64, radius: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  trace_var!(radius);
  let result = (mass / radius).sqrt();
  trace_var!(result);
  trace_exit!();
  result
}
