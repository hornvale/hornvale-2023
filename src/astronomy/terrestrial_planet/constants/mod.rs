/// Minimum mass for a terrestrial planet, in Mearth.
pub const MINIMUM_MASS: f64 = 0.1;

/// Maximum mass for a terrestrial planet, in Mearth.
pub const MAXIMUM_MASS: f64 = 10.0;

/// Minimum mass for a habitable planet, in Mearth.
/// Raised from 0.10 because that sounds ludicrous.
pub const MINIMUM_HABITABLE_MASS: f64 = 0.75;

/// Maximum mass for a habitable planet, in Mearth.
/// Lowered because 3.5 just sounds extreme to me.
pub const MAXIMUM_HABITABLE_MASS: f64 = 1.50;

/// Minimum habitable rotational period, in Dearth.
pub const MINIMUM_HABITABLE_ROTATIONAL_PERIOD: f64 = 0.25;

/// Maximum habitable rotational period, in Dearth.
pub const MAXIMUM_HABITABLE_ROTATIONAL_PERIOD: f64 = 2.0;

/// Minimum orbitable eccentricity.
pub const MINIMUM_ORBITAL_ECCENTRICITY: f64 = 0.0;

/// Maximum orbitable eccentricity.
pub const MAXIMUM_ORBITAL_ECCENTRICITY: f64 = 0.10;

/// Maximum habitable orbitable eccentricity.
pub const MINIMUM_HABITABLE_ORBITAL_ECCENTRICITY: f64 = MINIMUM_ORBITAL_ECCENTRICITY;

/// Maximum habitable orbitable eccentricity.
pub const MAXIMUM_HABITABLE_ORBITAL_ECCENTRICITY: f64 = 0.02;

/// Minimum Bond albedo.
pub const MINIMUM_BOND_ALBEDO: f64 = 0.01;

/// Maximum Bond albedo.
pub const MAXIMUM_BOND_ALBEDO: f64 = 1.00;

/// Minimum Bond albedo.
pub const MINIMUM_HABITABLE_BOND_ALBEDO: f64 = 0.11;

/// Maximum Bond albedo.
pub const MAXIMUM_HABITABLE_BOND_ALBEDO: f64 = 0.50;

/// Stefan-Boltzmann constant.
pub const STEFAN_BOLTZMANN_CONSTANT: f64 = 0.00005670374419;

/// Too damned cold.
pub const MINIMUM_HABITABLE_TEMPERATURE: f64 = 273.0;

/// Too damned hot.
pub const MAXIMUM_HABITABLE_TEMPERATURE: f64 = 323.0;

/// Too damned floaty.
pub const MINIMUM_HABITABLE_GRAVITY: f64 = 0.5;

/// Too damned hard to get out of bed.
pub const MAXIMUM_HABITABLE_GRAVITY: f64 = 1.5;
