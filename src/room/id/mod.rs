use uuid::Uuid;

/// The `Id` type.
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
pub struct Id(pub String);

impl Default for Id {
  fn default() -> Self {
    let id = Uuid::new_v4().to_string();
    Self(id)
  }
}
