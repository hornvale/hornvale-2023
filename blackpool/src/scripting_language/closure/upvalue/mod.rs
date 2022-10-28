use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::value::Value;

/// What's upvalue?  Nawmuch, man, what's up with you?
#[derive(Clone, Debug, Display)]
#[display(fmt = "location: {}, closed: {:#?}", location, closed)]
pub struct Upvalue {
  /// The index of this item.
  pub location: usize,
  /// A closed-over value.
  pub closed: Option<Value>,
}

impl Upvalue {
  /// Constructor.
  #[named]
  pub fn new(location: usize) -> Self {
    trace_enter!();
    trace_var!(location);
    let closed = None;
    trace_var!(closed);
    let result = Upvalue { location, closed };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Upvalue {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let result = write!(f, "upvalue");
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<Upvalue>();
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    trace_var!(garbage_collector);
    if let Some(object) = self.closed {
      garbage_collector.mark_value(object);
    }
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
