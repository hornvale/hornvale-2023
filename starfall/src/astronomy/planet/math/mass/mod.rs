pub const EARTH_MASS_PER_JUPITER_MASS: f64 = 317.8;

/// Convert from Mearth to Mjupiter.
#[named]
pub fn earth_mass_to_jupiter_mass(mass: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  let result = mass / EARTH_MASS_PER_JUPITER_MASS;
  trace_var!(result);
  trace_exit!();
  result
}

/// Convert from Mjupiter to Mearth.
#[named]
pub fn jupiter_mass_to_earth_mass(mass: f64) -> f64 {
  trace_enter!();
  trace_var!(mass);
  let result = mass * EARTH_MASS_PER_JUPITER_MASS;
  trace_var!(result);
  trace_exit!();
  result
}
