use std::f64::consts::PI;

use crate::astronomy::star::math::distance::METERS_PER_AU;
use crate::astronomy::star::math::luminosity::ERGS_PER_SEC_PER_LSOL;
use crate::astronomy::terrestrial_planet::constants::*;

pub const GREENHOUSE_EFFECT: f64 = 0.5841;

/// Calculate the equilibrium temperature for a planet based on the host star's
/// luminosity, distance, etc.
/// Answer in Kelvin.
#[named]
pub fn get_equilibrium_temperature(
  bond_albedo: f64,
  greenhouse_effect: f64,
  star_luminosity: f64,
  star_distance: f64,
) -> f64 {
  trace_enter!();
  trace_var!(bond_albedo);
  trace_var!(greenhouse_effect);
  trace_var!(star_luminosity);
  trace_var!(star_distance);
  let luminosity = star_luminosity * ERGS_PER_SEC_PER_LSOL;
  trace_var!(luminosity);
  let distance = star_distance * METERS_PER_AU * 100.0;
  trace_var!(distance);
  let t_greenhouse = greenhouse_effect * GREENHOUSE_EFFECT;
  trace_var!(t_greenhouse);
  let absorption = ((1.0 - bond_albedo) * luminosity / (16.0 * PI * STEFAN_BOLTZMANN_CONSTANT)).sqrt();
  trace_var!(absorption);
  let t_effective = absorption.sqrt() * (1.0 / distance.sqrt());
  trace_var!(t_effective);
  let t_equilibrium = t_effective.powf(4.0) * (1.0 + (3.0 * t_greenhouse / 4.0));
  trace_var!(t_equilibrium);
  let t_surface = t_equilibrium / 0.9;
  trace_var!(t_surface);
  let result = t_surface.powf(1.0 / 4.0);
  trace_var!(result);
  trace_exit!();
  result
}
