use crate::astronomy::_type::*;

/// Get the habitable zone of a star (in AU) based on its luminosity (in Lsol).
pub fn star_luminosity_to_habitable_zone(luminosity: LSol) -> (LAu, LAu) {
  (LAu((luminosity.0 / 1.1).sqrt()), LAu((luminosity.0 / 0.53).sqrt()))
}
