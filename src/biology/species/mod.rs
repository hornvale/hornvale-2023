pub mod collection;
pub use collection::SpeciesCollection;

/// The `Species` enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Species {
  /// The snake-cased, unique ID of this species.
  pub id: String,
  /// This is lowercase, reflecting taxonomic convention.
  pub name: String,
  /// The genus to which this species belongs.
  pub genus: String,
}
