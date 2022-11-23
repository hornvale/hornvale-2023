use crate::astronomy::_type::*;
use crate::astronomy::star::math::spectral_class::{
  get_random_habitable_spectral_class, get_random_spectral_class, spectral_class_to_habitable_mass_range,
  spectral_class_to_mass_range,
};
use rand::prelude::*;

/// Get a (weighted) random mass for a star.
pub fn get_random_stellar_mass<R: Rng + ?Sized>(rng: &mut R) -> MSol {
  let spectral_class = get_random_spectral_class(rng);
  let mass_range = spectral_class_to_mass_range(spectral_class);
  MSol(rng.gen_range(mass_range))
}

/// Get a (weighted) habitable random mass for a star.
pub fn get_random_habitable_stellar_mass<R: Rng + ?Sized>(rng: &mut R) -> MSol {
  let spectral_class = get_random_habitable_spectral_class(rng);
  let mass_range = spectral_class_to_habitable_mass_range(spectral_class);
  MSol(rng.gen_range(mass_range))
}
