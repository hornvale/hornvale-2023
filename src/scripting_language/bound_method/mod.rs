use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::closure::Closure;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::value::Value;

/// The `BoundMethod` type.
#[derive(Debug)]
pub struct BoundMethod {
  /// The receiver of the call.
  pub receiver: Value,
  /// The method being called.
  pub method: Reference<Closure>,
}

impl BoundMethod {
  /// Constructor.
  #[named]
  pub fn new(receiver: Value, method: Reference<Closure>) -> Self {
    BoundMethod { receiver, method }
  }
}

impl Trace for BoundMethod {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let method = garbage_collector.deref(self.method);

    method.format(f, garbage_collector)
  }
  #[named]
  fn get_size(&self) -> usize {
    size_of::<BoundMethod>()
  }
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_value(self.receiver);
    garbage_collector.mark_object(self.method);
  }
  #[named]
  fn as_any(&self) -> &dyn Any {
    self as _
  }
  #[named]
  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
