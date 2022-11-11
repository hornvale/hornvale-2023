use std::fmt::{Display, Formatter as FmtFormatter, Result as FmtResult};

use crate::scripting::garbage_collection::collector::Collector;
use crate::scripting::garbage_collection::trace::Trace;

/// The `Formatter` type.
pub struct Formatter<'garbage, T: Trace> {
  /// A garbage collector.
  pub collector: &'garbage Collector,
  /// The inner object.
  pub object: T,
}

impl<'garbage, T: Trace> Formatter<'garbage, T> {
  pub fn new(object: T, collector: &'garbage Collector) -> Self {
    Formatter { object, collector }
  }
}

impl<'garbage, T: Trace> Display for Formatter<'garbage, T> {
  fn fmt(&self, f: &mut FmtFormatter) -> FmtResult {
    self.object.format(f, self.collector)
  }
}
