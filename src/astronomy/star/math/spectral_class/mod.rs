use crate::astronomy::_type::*;
use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::temperature::star_mass_to_temperature;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::ops::Range;

/// Get a (weighted) random spectral class.
pub fn get_random_spectral_class<R: Rng + ?Sized>(rng: &mut R) -> char {
  let choices = ['O', 'B', 'A', 'F', 'G', 'K', 'M'];
  let weights = [
    CLASS_O_WEIGHT,
    CLASS_B_WEIGHT,
    CLASS_A_WEIGHT,
    CLASS_F_WEIGHT,
    CLASS_G_WEIGHT,
    CLASS_K_WEIGHT,
    CLASS_M_WEIGHT,
  ];
  let dist = WeightedIndex::new(&weights).unwrap();

  choices[dist.sample(rng)]
}

/// Get a (weighted) random habitable spectral class.
pub fn get_random_habitable_spectral_class<R: Rng + ?Sized>(rng: &mut R) -> char {
  let choices = ['F', 'G', 'K'];
  let weights = [CLASS_F_WEIGHT, CLASS_G_WEIGHT, CLASS_K_WEIGHT];
  let dist = WeightedIndex::new(&weights).unwrap();

  choices[dist.sample(rng)]
}

/// Get a mass range from a specified spectral class.
pub fn spectral_class_to_mass_range(char: char) -> Range<f64> {
  match char {
    'o' | 'O' => 16.0..MAXIMUM_MASS.0,
    'b' | 'B' => 2.1..16.0,
    'a' | 'A' => 1.4..2.1,
    'f' | 'F' => 1.04..1.4,
    'g' | 'G' => 0.8..1.04,
    'k' | 'K' => 0.45..0.8,
    'm' | 'M' => MINIMUM_MASS.0..0.45,
    _ => unreachable!(),
  }
}

/// Get a mass range from a specified spectral class.
pub fn spectral_class_to_habitable_mass_range(char: char) -> Range<f64> {
  match char {
    'f' | 'F' => 1.04..MAXIMUM_HABITABLE_MASS.0,
    'g' | 'G' => 0.8..1.04,
    'k' | 'K' => MINIMUM_HABITABLE_MASS.0..0.8,
    _ => unreachable!(),
  }
}

/// Get the spectral class of a main-sequence star in Kelvin based on its Msol.
pub fn star_mass_to_spectral_class(mass: MSol) -> Result<String, Error> {
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let temperature = star_mass_to_temperature(mass)?.0;
  let spectral_type = match temperature {
    temperature if temperature < 3_700.0 => 'M',
    temperature if temperature < 5_200.0 => 'K',
    temperature if temperature < 6_000.0 => 'G',
    temperature if temperature < 7_500.0 => 'F',
    temperature if temperature < 10_000.0 => 'A',
    temperature if temperature < 33_000.0 => 'B',
    temperature if temperature < 95_000.0 => 'O',
    _ => unreachable!(),
  };
  let decile = match temperature {
    temperature if temperature < 3_700.0 => 10.0 * (1.0 - ((temperature - 2_000.0) / 1_700.0)),
    temperature if temperature < 5_200.0 => 10.0 * (1.0 - ((temperature - 3_700.0) / 1_500.0)),
    temperature if temperature < 6_000.0 => 10.0 * (1.0 - ((temperature - 5_200.0) / 800.0)),
    temperature if temperature < 7_500.0 => 10.0 * (1.0 - ((temperature - 6_000.0) / 1_500.0)),
    temperature if temperature < 10_000.0 => 10.0 * (1.0 - ((temperature - 7_500.0) / 2_500.0)),
    temperature if temperature < 33_000.0 => 10.0 * (1.0 - ((temperature - 10_000.0) / 23_000.0)),
    temperature if temperature < 95_000.0 => 10.0 * (1.0 - ((temperature - 33_000.0) / 62_000.0)),
    _ => unreachable!(),
  };
  let result = format!("{}{:.0}V", spectral_type, decile);

  Ok(result)
}
