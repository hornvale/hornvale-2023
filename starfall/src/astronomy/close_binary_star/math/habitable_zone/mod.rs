use crate::astronomy::star::Star;

/// Calculate the habitable zone of a close binary system.
#[named]
pub fn get_habitable_zone(star1: &Star, star2: &Star) -> (f64, f64) {
  trace_enter!();
  let luminosity = star1.luminosity + star2.luminosity;
  trace_var!(luminosity);
  let inner_bound = (luminosity / 1.1).sqrt();
  trace_var!(inner_bound);
  let outer_bound = (luminosity / 0.53).sqrt();
  trace_var!(outer_bound);
  let result = (inner_bound, outer_bound);
  trace_var!(result);
  trace_exit!();
  result
}
