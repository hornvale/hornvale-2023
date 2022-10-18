const YEARS_PER_GIGAYEAR: f64 = 1_000_000_000.0;

/// Years -> Gyr
pub fn years_to_gyr(years: f64) -> f64 {
  years / YEARS_PER_GIGAYEAR
}

/// Gyr -> Years
pub fn gyr_to_years(gyr: f64) -> f64 {
  gyr * YEARS_PER_GIGAYEAR
}
