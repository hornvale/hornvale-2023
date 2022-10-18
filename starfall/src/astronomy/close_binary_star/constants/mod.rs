use crate::astronomy::star::constants::MAXIMUM_MASS as MAXIMUM_STAR_MASS;
use crate::astronomy::star::constants::MINIMUM_MASS as MINIMUM_STAR_MASS;

/// The minimum separation of binary stars, in AU.
pub const MINIMUM_SEPARATION: f64 = 0.04;

/// The minimum average separation of "close" binary stars, in AU.
pub const MINIMUM_AVERAGE_SEPARATION: f64 = 0.1;

/// The maximum average separation of "close" binary stars, in AU.
pub const MAXIMUM_AVERAGE_SEPARATION: f64 = 6.0;

/// The minimum orbital eccentricity of "close" binary stars (unitless).
pub const MINIMUM_ORBITAL_ECCENTRICITY: f64 = 0.1;

/// The maximum orbital eccentricity of "close" binarsy stars (unitless).
pub const MAXIMUM_ORBITAL_ECCENTRICITY: f64 = 0.7;

/// The minimum combined mass of a binary system.
/// Set it to 4 * minimum main-sequence star mass.
/// We don't want it to be too small.
pub const MINIMUM_COMBINED_MASS: f64 = 4.0 * MINIMUM_STAR_MASS;

/// The maximum combined mass of a binary system.
/// Set it to maximum main-sequence star mass.
/// We don't need binary supergiants.
pub const MAXIMUM_COMBINED_MASS: f64 = MAXIMUM_STAR_MASS;

/// The minimum individual mass of a binary system member.
/// Set it to 1 * minimum main-sequence star mass.
pub const MINIMUM_INDIVIDUAL_MASS: f64 = MINIMUM_STAR_MASS;

/// The maximum individual mass of a binary system member.
/// Set it to 1 * maximum main-sequence star mass.
pub const MAXIMUM_INDIVIDUAL_MASS: f64 = MAXIMUM_STAR_MASS;

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_HABITABLE_AGE: f64 = 4.0;

/// The minimum habitable average separation of "close" binary stars, in AU.
pub const MINIMUM_HABITABLE_AVERAGE_SEPARATION: f64 = 0.1;

/// The maximum habitable average separation of "close" habitable binary stars,
/// in AU.
/// I dropped this down from ~6AU because this just was not happening.
pub const MAXIMUM_HABITABLE_AVERAGE_SEPARATION: f64 = 0.4;

/// The minimum orbital eccentricity of "close" binary stars (unitless).
pub const MINIMUM_HABITABLE_ORBITAL_ECCENTRICITY: f64 = 0.2;

/// The maximum orbital eccentricity of "close" binary stars (unitless).
pub const MAXIMUM_HABITABLE_ORBITAL_ECCENTRICITY: f64 = 0.6;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_COMBINED_MASS: f64 = 1.0;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_COMBINED_MASS: f64 = 2.0;

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_INDIVIDUAL_MASS: f64 = 0.1;

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_INDIVIDUAL_MASS: f64 = 1.25;
