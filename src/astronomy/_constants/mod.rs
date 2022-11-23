use crate::astronomy::_type::*;

/// The probability that a given star subsystem will be binary.
///
/// This probability might be slightly lower than actual.
pub const BINARY_STAR_PROBABILITY: f64 = 0.25;

/// Kilograms per solar mass.
pub const KG_PER_SOLAR_MASS: MKg = MKg(1.989E30);

/// Kilograms per Jupiter mass.
pub const KG_PER_JUPITER_MASS: MKg = MKg(5.26704E28);

/// Kilograms per earth mass.
pub const KG_PER_EARTH_MASS: MKg = MKg(5.972E24);

/// Kilograms per lunar mass.
pub const KG_PER_LUNAR_MASS: MKg = MKg(7.34767309E22);

/// Kilometers in RJupiter.
pub const KM_PER_JUPITER_RADIUS: LKm = LKm(69_911.0);

/// Kilometers in REarth.
pub const KM_PER_EARTH_RADIUS: LKm = LKm(6371.0);

/// Diameter of the Earth in Kilometers.
pub const KM_PER_EARTH_DIAMETER: LKm = LKm(2.0 * KM_PER_EARTH_RADIUS.0);

/// Minimum mass, in Mluna.
pub const MINIMUM_MOON_MASS: MLuna = MLuna(0.05);

/// Maximum mass, in Mluna.
pub const MAXIMUM_MOON_MASS: MLuna = MLuna(1.00);

/// Minimum albedo (unitless).
pub const MINIMUM_MOON_ALBEDO: f64 = 0.25;

/// Maximum albedo (unitless).
pub const MAXIMUM_MOON_ALBEDO: f64 = 1.00;

/// Ratio of Luna's share of the Earth-Luna gravitational parameter.
pub const LUNA_GRAVITATIONAL_PARAMETER_SHARE: f64 = 0.0123;

/// Ratio of Earth mass to solar mass.
pub const EARTH_MASS_PER_SOLAR_MASS: MEarth = MEarth(333_000.0);

/// Ratio of Jupiter mass to solar mass.
pub const JUPITER_MASS_PER_SOLAR_MASS: MJupiter = MJupiter(1048.0);

/// Ratio of Earth mass to Jupiter mass.
pub const EARTH_MASS_PER_JUPITER_MASS: MEarth = MEarth(317.8);

/// Ratio of Luna mass to Earth mass.
pub const LUNA_MASS_PER_EARTH_MASS: MLuna = MLuna(81.3);

/// Jupiter's density.
pub const DENSITY_OF_JUPITER: DGmCm3 = DGmCm3(1.33);

/// Earth's density.
pub const DENSITY_OF_EARTH: DGmCm3 = DGmCm3(5.51);

/// Luna's density.
pub const DENSITY_OF_LUNA: DGmCm3 = DGmCm3(3.34);

/// Sol's density.
pub const DENSITY_OF_SOL: DGmCm3 = DGmCm3(1.41);

/// Hours per day.
pub const EARTH_HOURS_PER_DAY: TEarthHour = TEarthHour(24.0);

/// Days per year.
pub const EARTH_DAYS_PER_YEAR: TEarthDay = TEarthDay(365.25);

/// LSol -> Ergs/sec
pub const ERGS_PER_SEC_PER_LSOL: f64 = 3.846E33;

/// LSol -> Joules/sec
pub const JOULES_PER_SEC_PER_LSOL: f64 = 3.846E26;

/// The radius of our stellar neighborhood.
///
/// This may be flexible or changed at some point, but for the time being I'm
/// thinking about fairly conventional fantasy systems where interstellar
/// travel isn't a thing.
///
/// Measured in Ly, or light years.
pub const STELLAR_NEIGHBORHOOD_RADIUS: LLyr = LLyr(10.0);

/// The stellar density of our (stellar) neighborhood.
///
/// As above, this is currently set to be fairly conventional.
///
/// Measured in s/ly^3, or stars per cubic light year.
pub const STELLAR_NEIGHBORHOOD_DENSITY: f64 = 0.004;

/// The minimum number of moons we'll generate for a terrestrial planet.
pub const MINIMUM_TERRESTRIAL_MOONS: usize = 0;

/// The maximum number of moons we'll generate for a terrestrial planet.
pub const MAXIMUM_TERRESTRIAL_MOONS: usize = 2;

/// The minimum number of moons we'll generate for a gas giant plant.
pub const MINIMUM_GAS_GIANT_MOONS: usize = 8;

/// The maximum number of moons we'll generate for a gas giant plant.
pub const MAXIMUM_GAS_GIANT_MOONS: usize = 20;

/// Minimum number of satellite systems to generate.
pub const MINIMUM_SATELLITE_SYSTEMS: usize = 0;

/// Maximum number of satellite systems to generate.
pub const MAXIMUM_SATELLITE_SYSTEMS: usize = 12;
