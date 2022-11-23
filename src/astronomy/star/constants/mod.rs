use crate::astronomy::_type::*;

/// Below this is too low for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_MASS: MSol = MSol(0.075);

/// Above this is too high for a main-sequence star, probably.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_MASS: MSol = MSol(120.0);

/// Below this is probably too low to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MINIMUM_HABITABLE_MASS: MSol = MSol(0.55);

/// Above this is probably too high to support conventional life.
/// Measured in Msol, or solar mass equivalents.
pub const MAXIMUM_HABITABLE_MASS: MSol = MSol(1.25);

/// Assume a star has to be at least this old to have interesting life.
///
/// I'm assuming that life could get started at least a little sooner than on
/// Earth, but figuring it'd take about the same amount of time to get to the
/// interesting parts.
///
/// Measured in Gyr, or billions of years.
pub const MINIMUM_HABITABLE_AGE: TGyr = TGyr(4.0);

/// The probability of generating an O-class star.
pub const CLASS_O_WEIGHT: f64 = 0.00003;

/// The probability of generating a B-class star.
pub const CLASS_B_WEIGHT: f64 = 0.13;

/// The probability of generating an A-class star.
pub const CLASS_A_WEIGHT: f64 = 0.6;

/// The probability of generating an F-class star.
pub const CLASS_F_WEIGHT: f64 = 3.0;

/// The probability of generating a G-class star.
pub const CLASS_G_WEIGHT: f64 = 7.6;

/// The probability of generating a K-class star.
pub const CLASS_K_WEIGHT: f64 = 12.1;

/// The probability of generating an M-class star.
pub const CLASS_M_WEIGHT: f64 = 76.45;
