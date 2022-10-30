/// Get the frost line of a star (in AU) based on its luminosity (in Lsol).
#[named]
pub fn star_luminosity_to_frost_line(luminosity: f64) -> f64 {
  trace_enter!();
  trace_var!(luminosity);
  let result = 4.85 * luminosity.sqrt();
  trace_var!(result);
  trace_exit!();
  result
}
