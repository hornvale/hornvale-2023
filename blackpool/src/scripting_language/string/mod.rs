use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::garbage_collection::collector::Collector;
use crate::scripting_language::garbage_collection::trace::Trace;

/// Implements the `Trace` trait for `String`.
impl Trace for String {
  #[named]
  fn format(&self, f: &mut Formatter, _garbage_collector: &Collector) -> FmtResult {
    trace_enter!();
    let result = write!(f, "{}", self);
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<String>() + self.as_bytes().len();
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Strings cannot reference other objects, so this part is easy.
  #[named]
  fn trace(&self, _garbage_collector: &mut Collector) {
    trace_enter!();
    trace_exit!();
  }

  #[named]
  fn as_any(&self) -> &dyn Any {
    trace_enter!();
    let result = self;
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn as_any_mut(&mut self) -> &mut dyn Any {
    trace_enter!();
    let result = self;
    trace_var!(result);
    trace_exit!();
    result
  }
}
