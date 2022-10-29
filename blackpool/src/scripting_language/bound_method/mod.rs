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
    trace_enter!();
    trace_var!(receiver);
    trace_var!(method);
    let result = BoundMethod { receiver, method };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for BoundMethod {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let method = garbage_collector.deref(self.method);
    trace_var!(method);
    let result = method.format(f, garbage_collector);
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<BoundMethod>();
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    garbage_collector.mark_value(self.receiver);
    garbage_collector.mark_object(self.method);
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
