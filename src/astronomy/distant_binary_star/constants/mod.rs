use crate::astronomy::_type::*;

/// The minimum average separation of "distant" binary stars, in AU.
pub const MINIMUM_AVERAGE_SEPARATION: LAu = LAu(120.0);

/// The maximum average separation of "distant" binary stars, in AU.
pub const MAXIMUM_AVERAGE_SEPARATION: LAu = LAu(600.0);

/// The minimum orbital eccentricity of "distant" binary stars (unitless).
pub const MINIMUM_ORBITAL_ECCENTRICITY: f64 = 0.4;

/// The maximum orbital eccentricity of "distant" binary stars (unitless).
pub const MAXIMUM_ORBITAL_ECCENTRICITY: f64 = 0.7;
