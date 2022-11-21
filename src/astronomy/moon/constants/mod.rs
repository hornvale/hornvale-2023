use crate::astronomy::_type::*;

/// Minimum mass, in Mluna.
pub const MINIMUM_MASS: MLuna = MLuna(0.05);

/// Maximum mass, in Mluna.
pub const MAXIMUM_MASS: MLuna = MLuna(1.00);

/// Minimum albedo (unitless).
pub const MINIMUM_ALBEDO: f64 = 0.25;

/// Maximum albedo (unitless).
pub const MAXIMUM_ALBEDO: f64 = 1.00;
