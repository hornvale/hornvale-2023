use crate::astronomy::star::constants::*;
use crate::astronomy::star::error::Error;
use crate::astronomy::star::math::temperature::star_mass_to_temperature;

/// Get the RGB color of a main-sequence star based on its Msol.
///
/// This is going to calculate the absolute RGB of the star, which is going to
/// be very pale and very subtly tinted.  To generate an apparent color, we are
/// going to have to account for atmospheric scattering.  The specifics of
/// that are going to depend upon the atmospheric characteristics of the planet
/// from which we are observing the star.
///
/// This came from StackOverflow: https://stackoverflow.com/q/21977786
#[named]
pub fn star_mass_to_rgb(mass: f64) -> Result<(u8, u8, u8), Error> {
  trace_enter!();
  trace_var!(mass);
  if mass <= MINIMUM_MASS {
    return Err(Error::MassTooLowForMainSequence);
  }
  if mass >= MAXIMUM_MASS {
    return Err(Error::MassTooHighForMainSequence);
  }
  let temperature = star_mass_to_temperature(mass)?;
  trace_var!(temperature);
  let x = match temperature {
    temperature if temperature >= 1_667.0 && temperature <= 4_000.0 => {
      ((-0.2661239 * (10.0_f64).powf(9.0)) / temperature.powf(3.0))
        + ((-0.2343580 * (10.0_f64).powf(6.0)) / temperature.powf(2.0))
        + ((0.8776956 * (10.0_f64).powf(3.0)) / temperature)
        + 0.179910
    },
    temperature if temperature >= 4_000.0 => {
      ((-3.0258469 * (10.0_f64).powf(9.0)) / temperature.powf(3.0))
        + ((2.1070379 * (10.0_f64).powf(6.0)) / temperature.powf(2.0))
        + ((0.2226347 * (10.0_f64).powf(3.0)) / temperature)
        + 0.240390
    },
    _ => 0.0,
  };
  trace_var!(x);
  let y = match temperature {
    temperature if temperature >= 1_667.0 && temperature <= 2_222.0 => {
      -1.1063814 * x.powf(3.0) - 1.34811020 * x.powf(2.0) + 2.18555832 * x - 0.20219683
    },
    temperature if temperature >= 2_222.0 && temperature <= 4_000.0 => {
      -0.9549476 * x.powf(3.0) - 1.37418593 * x.powf(2.0) + 2.09137015 * x - 0.16748867
    },
    temperature if temperature >= 4_000.0 => {
      3.0817580 * x.powf(3.0) - 5.87338670 * x.powf(2.0) + 3.75112997 * x - 0.37001483
    },
    _ => 0.0,
  };
  trace_var!(y);
  let y2 = if y == 0.0 { 0.0 } else { 1.0 };
  trace_var!(y2);
  let x2 = if y == 0.0 { 0.0 } else { (x * y2) / y };
  trace_var!(x2);
  let z2 = if y == 0.0 { 0.0 } else { ((1.0 - x - y) * y2) / y };
  trace_var!(z2);
  let r = 3.2406 * x2 - 1.5372 * y2 - 0.4986 * z2;
  trace_var!(r);
  let g = -0.9689 * x2 + 1.8758 * y2 + 0.0415 * z2;
  trace_var!(g);
  let b = 0.0557 * x2 - 0.2040 * y2 + 1.0570 * z2;
  trace_var!(b);
  let r2 = if r <= 0.0031308 {
    12.92 * r
  } else {
    1.055 * r.powf(1.0 / 2.4) - 0.055
  };
  trace_var!(r2);
  let g2 = if g <= 0.0031308 {
    12.92 * g
  } else {
    1.055 * g.powf(1.0 / 2.4) - 0.055
  };
  trace_var!(g2);
  let b2 = if b <= 0.0031308 {
    12.92 * b
  } else {
    1.055 * b.powf(1.0 / 2.4) - 0.055
  };
  trace_var!(b2);
  let result = ((r2 * 255.0) as u8, (g2 * 255.0) as u8, (b2 * 255.0) as u8);
  trace_var!(result);
  trace_3u8!(result);
  trace_exit!();
  Ok(result)
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_ms_star_mass_to_rgb() -> Result<(), Error> {
    init();
    trace_enter!();
    // Jolly ol' Sol
    let mut mass = 1.0;
    let mut expected = (255, 252, 245);
    let mut actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // M1V
    mass = 0.40;
    expected = (255, 241, 165);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // K9V
    mass = 0.50;
    expected = (255, 245, 185);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // G7V
    mass = 0.90;
    expected = (255, 251, 237);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // F6V
    mass = 1.20;
    expected = (255, 253, 255);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // A6V
    mass = 1.70;
    expected = (246, 254, 255);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // B5V
    mass = 8.0;
    expected = (223, 253, 255);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    // O8V
    mass = 25.0;
    expected = (217, 253, 255);
    actual = star_mass_to_rgb(mass)?;
    assert_eq!(expected, actual);
    trace_exit!();
    Ok(())
  }
}
