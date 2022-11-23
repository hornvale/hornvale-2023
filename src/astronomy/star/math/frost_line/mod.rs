use crate::astronomy::_type::*;

/// Get the frost line of a star (in AU) based on its luminosity (in Lsol).
pub fn star_luminosity_to_frost_line(luminosity: LSol) -> LAu {
  LAu(4.85 * luminosity.0.sqrt())
}
