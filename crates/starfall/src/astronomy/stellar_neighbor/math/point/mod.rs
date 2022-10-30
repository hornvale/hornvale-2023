use rand::prelude::*;
use std::f64::consts::PI;

/// Generate a random point in a unit sphere.
///
/// Obviously, I did not come up with this algorithm.
#[named]
pub fn get_random_point_in_sphere<R: Rng + ?Sized>(rng: &mut R) -> (f64, f64, f64) {
  trace_enter!();
  let u: f64 = rng.gen_range(0.0..1.0);
  trace_var!(u);
  let v: f64 = rng.gen_range(0.0..1.0);
  trace_var!(v);
  let theta = u * 2.0 * PI;
  trace_var!(theta);
  let phi = (2.0 * v - 1.0).acos();
  trace_var!(phi);
  let r = rng.gen_range(0.0_f64..1.0_f64).cbrt();
  trace_var!(r);
  let sin_theta = theta.sin();
  trace_var!(sin_theta);
  let cos_theta = theta.cos();
  trace_var!(cos_theta);
  let sin_phi = phi.sin();
  trace_var!(sin_phi);
  let cos_phi = phi.cos();
  trace_var!(cos_phi);
  let x = r * sin_phi * cos_theta;
  let y = r * sin_phi * sin_theta;
  let z = r * cos_phi;
  let result = (x, y, z);
  trace_var!(result);
  trace_exit!();
  result
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
    trace_enter!();
    let mut rng = thread_rng();
    trace_var!(rng);
    let point = get_random_point_in_sphere(&mut rng);
    info_var!(point);
    print_var!(point);
    trace_exit!();
  }
}
