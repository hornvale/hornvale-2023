use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::scripting::garbage_collection::trace::Trace;

/// The `Reference` type.
pub struct Reference<T: Trace> {
  /// The GC index of this reference.
  pub index: usize,
  /// Pretend that we own the thing.
  pub marker: PhantomData<T>,
}

/// Clone implementation.
impl<T: Trace> Clone for Reference<T> {
  #[inline]
  fn clone(&self) -> Reference<T> {
    *self
  }
}

/// Copy implementation.
impl<T: Trace> Copy for Reference<T> {}

/// Debug implementation.
impl<T: Trace> Debug for Reference<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap();
    let result = write!(f, "ref({}:{})", self.index, full_name);
    result
  }
}

/// Display implementation.
impl<T: Trace> Display for Reference<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let result = write!(f, "{:?}", self);
    result
  }
}

/// Eq implementation.
impl<T: Trace> Eq for Reference<T> {}

/// Hash implementation.
impl Hash for Reference<String> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.index.hash(state);
  }
}

/// PartialEq implementation.
impl<T: Trace> PartialEq for Reference<T> {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index
  }
}
