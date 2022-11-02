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

  pub fn new(function: Reference<Function>) -> Self {
    let upvalues = Vec::new();

    Closure { function, upvalues }
  }
}

impl Trace for Closure {
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let function = garbage_collector.deref(self.function);

    function.format(f, garbage_collector)
  }

  fn get_size(&self) -> usize {
    size_of::<Closure>() + self.upvalues.capacity() * size_of::<Reference<Upvalue>>()
  }

  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_object(self.function);
    for &upvalue in &self.upvalues {
      garbage_collector.mark_object(upvalue);
    }
  }

  fn as_any(&self) -> &dyn Any {
    self as _
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
