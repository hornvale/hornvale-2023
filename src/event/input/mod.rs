/// The `InputEvent` type.
///
/// This represents a piece of textual input received from a player.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Input {
  pub input: String,
}
