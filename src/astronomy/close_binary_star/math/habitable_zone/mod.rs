use crate::astronomy::_type::*;
use crate::astronomy::star::Star;

/// Calculate the habitable zone of a close binary system.
pub fn get_habitable_zone(star1: &Star, star2: &Star) -> (LAu, LAu) {
  let luminosity = star1.luminosity + star2.luminosity;
  let inner_bound = LAu((luminosity / 1.1).0.sqrt());
  let outer_bound = LAu((luminosity / 0.53).0.sqrt());
  (inner_bound, outer_bound)
}
