/// What's upvalue?  Nawmuch, man, what's up with you?
#[derive(Clone, Debug, Display, Eq, Hash, PartialEq)]
#[display(fmt = "index: {}, is_local: {}", index, is_local)]
pub struct Upvalue {
  /// The index of this item.
  pub index: u16,
  /// Whether this is local or not.
  pub is_local: bool,
}
