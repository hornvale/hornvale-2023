use crate::astronomy::star::Star;

/// Calculate the frost line of a close binary system.
#[named]
pub fn get_frost_line(star1: &Star, star2: &Star) -> f64 {
  trace_enter!();
  let luminosity = star1.luminosity + star2.luminosity;
  trace_var!(luminosity);
  let result = 4.85 * luminosity.sqrt();
  trace_var!(result);
  trace_exit!();
  result
}
