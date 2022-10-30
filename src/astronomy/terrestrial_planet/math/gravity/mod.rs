/// Calculate the gravity of a terrestrial planet, given its mass and radius.
///
/// Units are Mearth, Rearth, and Gearth.
#[named]
pub fn get_gravity(mass: f64, radius: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  trace_var!(radius);
  let result = mass / radius.powf(2.0);
  trace_var!(result);
  trace_exit!();
  result
}
