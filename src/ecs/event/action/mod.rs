use crate::action::Action as ActionObject;

/// The `ActionEvent` type.
///
/// This represents a action attempted by any entity.
#[derive(Clone, Debug)]
pub struct Action {
  pub action: ActionObject,
}
