/// The `Script` type.
///
/// This represents a piece of text passed to the scripting engine.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Script {
  pub string: String,
}
