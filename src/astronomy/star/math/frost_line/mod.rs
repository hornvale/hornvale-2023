/// Get the frost line of a star (in AU) based on its luminosity (in Lsol).

pub fn star_luminosity_to_frost_line(luminosity: f64) -> f64 {
  4.85 * luminosity.sqrt()
}
