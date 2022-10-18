const GRAMS_PER_CM3_PER_DSOL: f64 = 1.41;

/// g/cm^3 -> Dsol
pub fn grams_to_dsol(grams: f64) -> f64 {
  grams / GRAMS_PER_CM3_PER_DSOL
}

/// Dsol -> g/cm^3
pub fn dsol_to_grams(dsol: f64) -> f64 {
  dsol * GRAMS_PER_CM3_PER_DSOL
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_grams_to_dsol() {
    init();
    trace_enter!();
    let expected = 1.0;
    let actual = grams_to_dsol(1.41);
    info_var!(actual);
    assert_approx_eq!(expected, actual, 1E-3);
    print_var!(actual);
    trace_exit!();
  }
}
