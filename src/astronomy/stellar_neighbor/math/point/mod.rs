use rand::prelude::*;
use std::f64::consts::PI;

/// Generate a random point in a unit sphere.
///
/// Obviously, I did not come up with this algorithm.
#[named]
pub fn get_random_point_in_sphere<R: Rng + ?Sized>(rng: &mut R) -> (f64, f64, f64) {
  let u: f64 = rng.gen_range(0.0..1.0);

  let v: f64 = rng.gen_range(0.0..1.0);

  let theta = u * 2.0 * PI;

  let phi = (2.0 * v - 1.0).acos();

  let r = rng.gen_range(0.0_f64..1.0_f64).cbrt();

  let sin_theta = theta.sin();

  let cos_theta = theta.cos();

  let sin_phi = phi.sin();

  let cos_phi = phi.cos();

  let x = r * sin_phi * cos_theta;
  let y = r * sin_phi * sin_theta;
  let z = r * cos_phi;

  (x, y, z)
}

#[cfg(test)]
pub mod test {

  use rand::prelude::*;

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_get_random_point_in_sphere() {
    init();

    let mut rng = thread_rng();

    let point = get_random_point_in_sphere(&mut rng);
    info_var!(point);
    print_var!(point);
  }
}
