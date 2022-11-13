use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting::garbage_collection::reference::Reference;
use crate::scripting::garbage_collection::trace::Trace;
use crate::scripting::table::Table;

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
  pub fn new(name: Reference<String>) -> Self {
    let methods = Table::new();
    Class { name, methods }
  }
}

impl Trace for Class {
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    let name = garbage_collector.deref(self.name);
    let result = write!(f, "{}", name);
    result
  }

  fn get_size(&self) -> usize {
    size_of::<Class>()
  }

  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    garbage_collector.mark_object(self.name);
    garbage_collector.mark_table(&self.methods);
  }

  fn as_any(&self) -> &dyn Any {
    self as _
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self as _
  }
}
