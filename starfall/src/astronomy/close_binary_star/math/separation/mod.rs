#[named]
pub fn get_minimum_separation(min_distances: (f64, f64)) -> f64 {
  trace_enter!();
  trace_var!(min_distances);
  let result = min_distances.0 + min_distances.1;
  trace_var!(result);
  trace_exit!();
  result
}

#[named]
pub fn get_maximum_separation(max_distances: (f64, f64)) -> f64 {
  trace_enter!();
  trace_var!(max_distances);
  let result = max_distances.0 + max_distances.1;
  trace_var!(result);
  trace_exit!();
  result
}
