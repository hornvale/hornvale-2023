use crate::astronomy::_type::*;

pub const OXYGEN_WEIGHT: f64 = 0.032;
pub const CO2_WEIGHT: f64 = 0.044;
pub const ARGON_WEIGHT: f64 = 0.04;
pub const NITROGEN_WEIGHT: f64 = 0.028;

/// Calculates whether a molecule can be stable in this atmosphere, given:
/// `equilibrium_temperature` - of the body, in Kelvin.
/// `escape_velocity` - of the body, in Vearth.
/// `mol_weight` - the weight of the molecule in kg/mol.
pub fn get_molecule_stability(mol_weight: f64, equilibrium_temperature: TKel, escape_velocity: f64) -> f64 {
  ((3.0 * 8.3145 * (equilibrium_temperature.0 / 288.0) * 1500.0) / mol_weight).sqrt()
    / ((escape_velocity * 11200.0) / 6.0)
}

/// Calculates whether a molecule can be stable in this atmosphere, given:
/// `equilibrium_temperature` - of the body, in Kelvin.
/// `escape_velocity` - of the body, in Vearth.
/// `mol_weight` - the weight of the molecule in kg/mol.
pub fn is_molecule_stable(mol_weight: f64, equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  let stability = get_molecule_stability(mol_weight, equilibrium_temperature, escape_velocity);
  stability < 1.0
}

pub fn get_oxygen_stability(equilibrium_temperature: TKel, escape_velocity: f64) -> f64 {
  let mol_weight = OXYGEN_WEIGHT;
  get_molecule_stability(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn get_carbon_dioxide_stability(equilibrium_temperature: TKel, escape_velocity: f64) -> f64 {
  let mol_weight = CO2_WEIGHT;
  get_molecule_stability(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn get_argon_stability(equilibrium_temperature: TKel, escape_velocity: f64) -> f64 {
  let mol_weight = ARGON_WEIGHT;
  get_molecule_stability(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn get_nitrogen_stability(equilibrium_temperature: TKel, escape_velocity: f64) -> f64 {
  let mol_weight = NITROGEN_WEIGHT;
  get_molecule_stability(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn is_oxygen_stable(equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  let mol_weight = OXYGEN_WEIGHT;
  is_molecule_stable(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn is_carbon_dioxide_stable(equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  let mol_weight = CO2_WEIGHT;
  is_molecule_stable(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn is_argon_stable(equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  let mol_weight = ARGON_WEIGHT;
  is_molecule_stable(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn is_nitrogen_stable(equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  let mol_weight = NITROGEN_WEIGHT;
  is_molecule_stable(mol_weight, equilibrium_temperature, escape_velocity)
}

pub fn is_atmospherically_stable(equilibrium_temperature: TKel, escape_velocity: f64) -> bool {
  is_oxygen_stable(equilibrium_temperature, escape_velocity)
    && is_carbon_dioxide_stable(equilibrium_temperature, escape_velocity)
    && is_argon_stable(equilibrium_temperature, escape_velocity)
    && is_nitrogen_stable(equilibrium_temperature, escape_velocity)
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[test]
  pub fn test_get_oxygen_stability() {
    init();
    let equilibrium_temperature = TKel(288.0);
    let escape_velocity = 1.0;
    let oxygen_stability = get_oxygen_stability(equilibrium_temperature, escape_velocity);
    assert_approx_eq!(oxygen_stability, 0.579, 0.001);
    assert!(is_oxygen_stable(equilibrium_temperature, escape_velocity));
  }

  #[test]
  pub fn test_get_carbon_dioxide_stability() {
    init();
    let equilibrium_temperature = TKel(288.0);
    let escape_velocity = 1.0;
    let carbon_dioxide_stability = get_carbon_dioxide_stability(equilibrium_temperature, escape_velocity);
    assert_approx_eq!(carbon_dioxide_stability, 0.494, 0.001);
    assert!(is_carbon_dioxide_stable(equilibrium_temperature, escape_velocity));
  }

  #[test]
  pub fn test_get_argon_stability() {
    init();
    let equilibrium_temperature = TKel(288.0);
    let escape_velocity = 1.0;
    let argon_stability = get_argon_stability(equilibrium_temperature, escape_velocity);
    assert_approx_eq!(argon_stability, 0.518, 0.001);
    assert!(is_argon_stable(equilibrium_temperature, escape_velocity));
  }

  #[test]
  pub fn test_get_nitrogen_stability() {
    init();
    let equilibrium_temperature = TKel(288.0);
    let escape_velocity = 1.0;
    let nitrogen_stability = get_nitrogen_stability(equilibrium_temperature, escape_velocity);
    assert_approx_eq!(nitrogen_stability, 0.619, 0.001);
    assert!(is_nitrogen_stable(equilibrium_temperature, escape_velocity));
  }
}
