pub fn get_minimum_separation(min_distances: (f64, f64)) -> f64 {
  min_distances.0 + min_distances.1
}

pub fn get_maximum_separation(max_distances: (f64, f64)) -> f64 {
  max_distances.0 + max_distances.1
}
