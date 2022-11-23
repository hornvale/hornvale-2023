use crate::astronomy::_type::*;

pub fn get_average_distances_from_barycenter(
  average_separation: LAu,
  primary_mass: MSol,
  secondary_mass: MSol,
) -> (LAu, LAu) {
  let combined_mass = primary_mass + secondary_mass;
  let d1 = LAu(average_separation.0 * (secondary_mass.0 / combined_mass.0));
  let d2 = LAu(average_separation.0 * (primary_mass.0 / combined_mass.0));

  (d1, d2)
}

pub fn get_minimum_distances_from_barycenter(
  average_separation: LAu,
  primary_mass: MSol,
  secondary_mass: MSol,
  orbital_eccentricity: f64,
) -> (LAu, LAu) {
  let average_distances = get_average_distances_from_barycenter(average_separation, primary_mass, secondary_mass);
  let d1 = LAu(average_distances.0 .0 * (1.0 - orbital_eccentricity));
  let d2 = LAu(average_distances.1 .0 * (1.0 - orbital_eccentricity));
  (d1, d2)
}

pub fn get_maximum_distances_from_barycenter(
  average_separation: LAu,
  primary_mass: MSol,
  secondary_mass: MSol,
  orbital_eccentricity: f64,
) -> (LAu, LAu) {
  let _combined_mass = primary_mass + secondary_mass;
  let average_distances = get_average_distances_from_barycenter(average_separation, primary_mass, secondary_mass);
  let d1 = average_distances.0 * (1.0 + orbital_eccentricity);
  let d2 = average_distances.1 * (1.0 + orbital_eccentricity);

  (d1, d2)
}
