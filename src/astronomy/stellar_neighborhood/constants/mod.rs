/// The radius of our stellar neighborhood.
///
/// This may be flexible or changed at some point, but for the time being I'm
/// thinking about fairly conventional fantasy systems where interstellar
/// travel isn't a thing.
///
/// Measured in Ly, or light years.
pub const STELLAR_NEIGHBORHOOD_RADIUS: f64 = 10.0;

/// The stellar density of our (stellar) neighborhood.
///
/// As above, this is currently set to be fairly conventional.
///
/// Measured in s/ly^3, or stars per cubic light year.
pub const STELLAR_NEIGHBORHOOD_DENSITY: f64 = 0.004;
