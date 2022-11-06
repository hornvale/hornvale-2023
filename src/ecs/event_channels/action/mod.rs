use crate::action::Action as ActionObject;

/// The `ActionEvent` type.
///
/// This represents a action attempted by any entity.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Action {
  pub action: ActionObject,
}
