/// Astronomical objects.
///
/// Organization, in a sort-of pseudo-BNF:
///
/// MOON = ()
/// MOONS = [MOON]
/// TERRESTRIAL_PLANET = ()
/// GAS_GIANT_PLANET = ()
/// PLANET = GAS_GIANT_PLANET | TERRESTRIAL_PLANET
/// SATELLITE_SYSTEM = (PLANET, MOONS)
/// SATELLITE_SYSTEMS = [SATELLITE_SYSTEM]
/// STAR = ()
/// CLOSE_BINARY_STAR = (STAR, STAR)
/// HOST_STAR = CLOSE_BINARY_STAR | STAR
/// PLANETARY_SYSTEM = (HOST_STAR, SATELLITE_SYSTEMS)
/// DISTANT_BINARY_STAR = (PLANETARY_SYSTEM, PLANETARY_SYSTEM)
/// STAR_SUBSYSTEM = DISTANT_BINARY_STAR | PLANETARY_SYSTEM
/// STAR_SYSTEM = (STAR_SUBSYSTEM)
/// STELLAR_NEIGHBOR = (STAR_SYSTEM)
/// STELLAR_NEIGHBORHOOD = [STELLAR_NEIGHBOR]
/// GALAXY = (STELLAR_NEIGHBORHOOD)
pub mod close_binary_star;
pub mod distant_binary_star;
pub mod galaxy;
pub mod gas_giant_planet;
pub mod host_star;
pub mod moon;
pub mod moons;
pub mod planet;
pub mod planetary_system;
pub mod satellite_system;
pub mod satellite_systems;
pub mod star;
pub mod star_subsystem;
pub mod star_system;
pub mod stellar_neighbor;
pub mod stellar_neighborhood;
pub mod terrestrial_planet;
