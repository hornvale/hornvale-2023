use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};

use crate::scripting::garbage_collection::collector::Collector;

pub mod formatter;

/// The `Trace` trait.
pub trait Trace {
  /// Format this object.
  fn format(&self, f: &mut Formatter, _gc: &Collector) -> FmtResult;
  /// Calculate the allocated size of this object.
  fn get_size(&self) -> usize;
  /// Mark objects.
  fn trace(&self, _gc: &mut Collector);
  /// Downcasting.
  fn as_any(&self) -> &dyn Any;
  /// Downcasting.
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
