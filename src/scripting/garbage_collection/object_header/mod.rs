use crate::scripting::garbage_collection::trace::Trace;

/// The `ObjectHeader` type.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct ObjectHeader {
  /// Whether this object is marked for collection.
  pub is_marked: bool,
  /// The allocated size of the object.
  pub size: usize,
  /// The object itself.
  #[derivative(Debug = "ignore")]
  pub object: Box<dyn Trace>,
}
