pub mod collection;
pub use collection::GenusCollection;

/// The `Genus` enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Genus {
  /// The unique ID for this genus, e.g. "goblin".  This is snake-cased.
  pub id: String,
  /// A proper name for this genus, e.g. "Goblin".  This is capitalized.
  pub name: String,
}
