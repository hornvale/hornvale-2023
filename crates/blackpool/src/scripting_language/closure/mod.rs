use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;

pub mod upvalue;
use upvalue::Upvalue;

/// The `Closure` type.
#[derive(Clone, Debug, Display)]
#[display(fmt = "function: {}, upvalues: {:#?}", function, upvalues)]
pub struct Closure {
  /// Internal function.
  pub function: Reference<Function>,
  /// Included upvalues.
  pub upvalues: Vec<Reference<Upvalue>>,
}

impl Closure {
  /// Constructor.
  #[named]
  pub fn new(function: Reference<Function>) -> Self {
    trace_enter!();
    trace_var!(function);
    let upvalues = Vec::new();
    trace_var!(upvalues);
    let result = Closure { function, upvalues };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Closure {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let function = garbage_collector.deref(self.function);
    let result = function.format(f, garbage_collector);
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<Closure>() + self.upvalues.capacity() * size_of::<Reference<Upvalue>>();
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    trace_var!(garbage_collector);
    garbage_collector.mark_object(self.function);
    for &upvalue in &self.upvalues {
      garbage_collector.mark_object(upvalue);
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
