use crate::astronomy::_type::*;
use crate::astronomy::star::Star;

/// Calculate the frost line of a close binary system in AU.
pub fn get_frost_line(star1: &Star, star2: &Star) -> LAu {
  let luminosity = star1.luminosity + star2.luminosity;
  LAu(4.85 * luminosity.0.sqrt())
}
