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
    let fields = Table::new();

    Instance { class, fields }
  }
}

impl Trace for Instance {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let class = garbage_collector.deref(self.class);

    let name = garbage_collector.deref(class.name);

    let result = write!(f, "{} instance", name);

    result
  }
  #[named]
  fn get_size(&self) -> usize {
    size_of::<Instance>() + self.fields.capacity() * (size_of::<Reference<String>>() + size_of::<Value>())
  }
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_object(self.class);
    garbage_collector.mark_table(&self.fields);
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
