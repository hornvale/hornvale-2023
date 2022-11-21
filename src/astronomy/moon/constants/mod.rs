use crate::astronomy::_type::*;

/// Minimum mass, in Mluna.
pub const MINIMUM_MASS: MLuna = MLuna(0.05);

/// Maximum mass, in Mluna.
pub const MAXIMUM_MASS: MLuna = MLuna(1.00);

/// Minimum albedo (unitless).
pub const MINIMUM_ALBEDO: f64 = 0.25;

/// Maximum albedo (unitless).
pub const MAXIMUM_ALBEDO: f64 = 1.00;

/// Ratio of Luna's share of the Earth-Luna gravitational parameter.
pub const LUNA_GRAVITATIONAL_PARAMETER_SHARE: f64 = 0.0123;

/// Diameter of the Earth in Kilometers.
pub const DIAMETER_EARTH_KM: f64 = 12_742.0;
