use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::class::Class;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::table::Table;
use crate::scripting_language::value::Value;

/// The `Instance` type.
#[derive(Debug)]
pub struct Instance {
  /// The class of which this is an instance.
  pub class: Reference<Class>,
  /// A table of fields and their values.
  pub fields: Table,
}

impl Instance {
  /// Constructor.
  #[named]
  pub fn new(class: Reference<Class>) -> Self {
    trace_enter!();
    trace_var!(class);
    let fields = Table::new();
    trace_var!(fields);
    let result = Instance { class, fields };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Instance {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let class = garbage_collector.deref(self.class);
    trace_var!(class);
    let name = garbage_collector.deref(class.name);
    trace_var!(name);
    let result = write!(f, "{} instance", name);
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<Instance>() + self.fields.capacity() * (size_of::<Reference<String>>() + size_of::<Value>());
    trace_var!(result);
    trace_exit!();
    result
  }
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    garbage_collector.mark_object(self.class);
    garbage_collector.mark_table(&self.fields);
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
