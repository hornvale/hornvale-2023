use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::garbage_collection::collector::Collector;
use crate::scripting_language::garbage_collection::trace::Trace;

/// Implements the `Trace` trait for `String`.
impl Trace for String {
  #[named]
  fn format(&self, f: &mut Formatter, _garbage_collector: &Collector) -> FmtResult {
    let result = write!(f, "{}", self);

    result
  }

  #[named]
  fn get_size(&self) -> usize {
    let result = size_of::<String>() + self.as_bytes().len();

    result
  }

  /// Strings cannot reference other objects, so this part is easy.
  #[named]
  fn trace(&self, _garbage_collector: &mut Collector) {}

  #[named]
  fn as_any(&self) -> &dyn Any {
    self as _
  }

  #[named]
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
