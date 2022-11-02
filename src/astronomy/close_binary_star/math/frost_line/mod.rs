use crate::astronomy::star::Star;

/// Calculate the frost line of a close binary system.
#[named]
pub fn get_frost_line(star1: &Star, star2: &Star) -> f64 {
  let luminosity = star1.luminosity + star2.luminosity;

  4.85 * luminosity.sqrt()
}
