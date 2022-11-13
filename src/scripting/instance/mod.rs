use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting::class::Class;
use crate::scripting::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting::garbage_collection::reference::Reference;
use crate::scripting::garbage_collection::trace::Trace;
use crate::scripting::table::Table;
use crate::scripting::value::Value;

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
  pub fn new(class: Reference<Class>) -> Self {
    let fields = Table::new();
    Instance { class, fields }
  }
}

impl Trace for Instance {
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let class = garbage_collector.deref(self.class);
    let name = garbage_collector.deref(class.name);
    let result = write!(f, "{} instance", name);
    result
  }

  fn get_size(&self) -> usize {
    size_of::<Instance>() + self.fields.capacity() * (size_of::<Reference<String>>() + size_of::<Value>())
  }

  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_object(self.class);
    garbage_collector.mark_table(&self.fields);
  }

  fn as_any(&self) -> &dyn Any {
    self as _
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
