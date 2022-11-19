/// The `Output` type.
///
/// This represents a piece of textual output to present to the player.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Output {
  pub string: String,
}
