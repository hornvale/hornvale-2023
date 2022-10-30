/// Get the habitable zone of a star (in AU) based on its luminosity (in Lsol).
#[named]
pub fn star_luminosity_to_habitable_zone(luminosity: f64) -> (f64, f64) {
  trace_enter!();
  trace_var!(luminosity);
  let result = ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt());
  trace_var!(result);
  trace_exit!();
  result
}
