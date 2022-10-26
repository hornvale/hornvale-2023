use std::any::type_name;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

use crate::scripting_language::garbage_collection::trace::Trace;

/// The `Reference` type.
pub struct Reference<T: Trace> {
  /// The GC index of this reference.
  pub index: usize,
  /// Pretend that we own the thing.
  pub marker: PhantomData<T>,
}

/// Clone implementation.
impl<T: Trace> Clone for Reference<T> {
  #[named]
  #[inline]
  fn clone(&self) -> Reference<T> {
    trace_enter!();
    let result = *self;
    trace_var!(result);
    trace_exit!();
    result
  }
}

/// Copy implementation.
impl<T: Trace> Copy for Reference<T> {}

/// Debug implementation.
impl<T: Trace> Debug for Reference<T> {
  #[named]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    trace_enter!();
    let full_name = type_name::<T>();
    trace_var!(full_name);
    full_name.split("::").last().unwrap();
    let result = write!(f, "ref({}:{})", self.index, full_name);
    trace_var!(result);
    trace_exit!();
    result
  }
}

/// Display implementation.
impl<T: Trace> Display for Reference<T> {
  #[named]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    trace_enter!();
    let result = write!(f, "{:?}", self);
    trace_var!(result);
    trace_exit!();
    result
  }
}

/// Eq implementation.
impl<T: Trace> Eq for Reference<T> {}

/// Hash implementation.
impl Hash for Reference<String> {
  #[named]
  fn hash<H: Hasher>(&self, state: &mut H) {
    trace_enter!();
    self.index.hash(state);
    trace_exit!();
  }
}

/// PartialEq implementation.
impl<T: Trace> PartialEq for Reference<T> {
  #[named]
  fn eq(&self, other: &Self) -> bool {
    trace_enter!();
    let result = self.index == other.index;
    trace_var!(result);
    trace_exit!();
    result
  }
}
