/// Get the habitable zone of a star (in AU) based on its luminosity (in Lsol).
#[named]
pub fn star_luminosity_to_habitable_zone(luminosity: f64) -> (f64, f64) {
  ((luminosity / 1.1).sqrt(), (luminosity / 0.53).sqrt())
}
