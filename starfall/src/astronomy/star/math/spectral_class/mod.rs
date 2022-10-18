use rand::distributions::WeightedIndex;
use rand::prelude::*;
use std::ops::Range;

use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::temperature::star_mass_to_temperature;

/// Get a (weighted) random spectral class.
#[named]
pub fn get_random_spectral_class<R: Rng + ?Sized>(rng: &mut R) -> char {
  trace_enter!();
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
  let result = choices[dist.sample(rng)];
  trace_var!(result);
  trace_exit!();
  result
}

/// Get a (weighted) random habitable spectral class.
#[named]
pub fn get_random_habitable_spectral_class<R: Rng + ?Sized>(rng: &mut R) -> char {
  trace_enter!();
  let choices = ['F', 'G', 'K'];
  let weights = [CLASS_F_WEIGHT, CLASS_G_WEIGHT, CLASS_K_WEIGHT];
  let dist = WeightedIndex::new(&weights).unwrap();
  let result = choices[dist.sample(rng)];
  trace_var!(result);
  trace_exit!();
  result
}

/// Get a mass range from a specified spectral class.
#[named]
pub fn spectral_class_to_mass_range(char: char) -> Range<f64> {
  trace_enter!();
  trace_var!(char);
  let result = match char {
    'o' | 'O' => 16.0..MAXIMUM_MASS,
    'b' | 'B' => 2.1..16.0,
    'a' | 'A' => 1.4..2.1,
    'f' | 'F' => 1.04..1.4,
    'g' | 'G' => 0.8..1.04,
    'k' | 'K' => 0.45..0.8,
    'm' | 'M' => MINIMUM_MASS..0.45,
    _ => unreachable!(),
  };
  trace_var!(result);
  trace_exit!();
  result
}

/// Get a mass range from a specified spectral class.
#[named]
pub fn spectral_class_to_habitable_mass_range(char: char) -> Range<f64> {
  trace_enter!();
  trace_var!(char);
  let result = match char {
    'f' | 'F' => 1.04..MAXIMUM_HABITABLE_MASS,
    'g' | 'G' => 0.8..1.04,
    'k' | 'K' => MINIMUM_HABITABLE_MASS..0.8,
    _ => unreachable!(),
  };
  trace_var!(result);
  trace_exit!();
  result
}

/// Get the spectral class of a main-sequence star in Kelvin based on its Msol.
#[named]
pub fn star_mass_to_spectral_class(mass: f64) -> Result<String, Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let temperature = star_mass_to_temperature(mass)?;
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
    temperature if temperature < 3_700.0 => (10.0 * (1.0 - ((temperature - 2_000.0) / 1_700.0))),
    temperature if temperature < 5_200.0 => (10.0 * (1.0 - ((temperature - 3_700.0) / 1_500.0))),
    temperature if temperature < 6_000.0 => (10.0 * (1.0 - ((temperature - 5_200.0) / 800.0))),
    temperature if temperature < 7_500.0 => (10.0 * (1.0 - ((temperature - 6_000.0) / 1_500.0))),
    temperature if temperature < 10_000.0 => (10.0 * (1.0 - ((temperature - 7_500.0) / 2_500.0))),
    temperature if temperature < 33_000.0 => (10.0 * (1.0 - ((temperature - 10_000.0) / 23_000.0))),
    temperature if temperature < 95_000.0 => (10.0 * (1.0 - ((temperature - 33_000.0) / 62_000.0))),
    _ => unreachable!(),
  };
  let result = format!("{}{}V", spectral_type, format!("{:.0}", decile));
  trace_var!(result);
  trace_exit!();
  Ok(result)
}
