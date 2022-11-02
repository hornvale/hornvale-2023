use crate::astronomy::star::Star;

/// Calculate the habitable zone of a close binary system.

pub fn get_habitable_zone(star1: &Star, star2: &Star) -> (f64, f64) {
  let luminosity = star1.luminosity + star2.luminosity;
  let inner_bound = (luminosity / 1.1).sqrt();
  let outer_bound = (luminosity / 0.53).sqrt();

  (inner_bound, outer_bound)
}
