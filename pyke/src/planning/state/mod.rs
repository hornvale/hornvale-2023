/// The `State` trait.
///
/// This represents some description of the world that an actor might use.
///
/// For instance, a very simple organism might track only their hunger in a
/// State object.  Then they might specify that they do not want to be hungry.
/// The resulting discrepancy can be used in planning to navigate the critter
/// to some food and eat it.
pub trait State {}
