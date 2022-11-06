use specs::prelude::*;

/// The `HasDescription` type.
#[derive(Clone, Component, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct HasDescription {
  /// Shown only on initial viewings.
  pub initial: Option<String>,
  /// A brief description, shown on subsequent views.
  pub brief: String,
}
