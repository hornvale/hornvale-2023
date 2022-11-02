#[named]
pub fn get_average_distances_from_barycenter(
  average_separation: f64,
  primary_mass: f64,
  secondary_mass: f64,
) -> (f64, f64) {
  let combined_mass = primary_mass + secondary_mass;

  let d1 = average_separation * (secondary_mass / combined_mass);
  let d2 = average_separation * (primary_mass / combined_mass);

  (d1, d2)
}

#[named]
pub fn get_minimum_distances_from_barycenter(
  average_separation: f64,
  primary_mass: f64,
  secondary_mass: f64,
  orbital_eccentricity: f64,
) -> (f64, f64) {
  let _combined_mass = primary_mass + secondary_mass;

  let average_distances = get_average_distances_from_barycenter(average_separation, primary_mass, secondary_mass);
  let d1 = average_distances.0 * (1.0 - orbital_eccentricity);
  let d2 = average_distances.1 * (1.0 - orbital_eccentricity);

  (d1, d2)
}

#[named]
pub fn get_maximum_distances_from_barycenter(
  average_separation: f64,
  primary_mass: f64,
  secondary_mass: f64,
  orbital_eccentricity: f64,
) -> (f64, f64) {
  let _combined_mass = primary_mass + secondary_mass;

  let average_distances = get_average_distances_from_barycenter(average_separation, primary_mass, secondary_mass);
  let d1 = average_distances.0 * (1.0 + orbital_eccentricity);
  let d2 = average_distances.1 * (1.0 + orbital_eccentricity);

  (d1, d2)
}
