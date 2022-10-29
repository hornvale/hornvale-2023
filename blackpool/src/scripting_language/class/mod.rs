use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::table::Table;

/// The `Class` type.
#[derive(Debug)]
pub struct Class {
  /// The name of this class.
  pub name: Reference<String>,
  /// A table of methods and their functions.
  pub methods: Table,
}

impl Class {
  /// Constructor.
  #[named]
  pub fn new(name: Reference<String>) -> Self {
    trace_enter!();
    trace_var!(name);
    let methods = Table::new();
    trace_var!(methods);
    let result = Class { name, methods };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Class {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let name = garbage_collector.deref(self.name);
    trace_var!(name);
    let result = write!(f, "{}", name);
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<Class>();
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    garbage_collector.mark_object(self.name);
    garbage_collector.mark_table(&self.methods);
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
