/// The `Strain` type.
///
/// Strains are defined programmatically; they should consist entirely of
/// modifications to the species genome.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Strain {
  /// The species ID of the parent species of this strain.
  pub species: String,
}
