pub const METERS_PER_SOLAR_RADIUS: f64 = 149_597_870_700.0;
pub const METERS_PER_AU: f64 = 1.496E11;

/// Rsol -> M
pub fn rsol_to_meters(radii: f64) -> f64 {
  radii * METERS_PER_SOLAR_RADIUS
}

/// M -> Rsol
pub fn meters_to_rsol(meters: f64) -> f64 {
  meters / METERS_PER_SOLAR_RADIUS
}

/// AU -> M
pub fn au_to_meters(au: f64) -> f64 {
  au * METERS_PER_AU
}

/// M -> AU
pub fn meters_to_au(meters: f64) -> f64 {
  meters / METERS_PER_AU
}
