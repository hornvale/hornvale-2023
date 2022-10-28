use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};
use std::mem::size_of;

use crate::scripting_language::chunk::Chunk;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::instruction::Instruction;
use crate::scripting_language::value::Value;

pub mod upvalue;
use upvalue::Upvalue;

/// The `Function` type.
#[derive(Clone, Debug, Display, PartialEq)]
#[display(
  fmt = "name: {}, chunk: {}, arity: {}, upvalues: {:#?}",
  name,
  chunk,
  arity,
  upvalues
)]
pub struct Function {
  /// The name of the function.
  pub name: Reference<String>,
  /// A chunk of code.
  pub chunk: Chunk,
  /// The arity of the function.
  pub arity: usize,
  /// Upvalues.
  pub upvalues: Vec<Upvalue>,
}

impl Function {
  /// Constructor.
  #[named]
  pub fn new(name: Reference<String>) -> Self {
    trace_enter!();
    trace_var!(name);
    let arity = 0;
    trace_var!(arity);
    let chunk = Chunk::default();
    trace_var!(chunk);
    let upvalues = Vec::new();
    trace_var!(upvalues);
    let result = Self {
      arity,
      chunk,
      name,
      upvalues,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Function {
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    let name = garbage_collector.deref(self.name);
    trace_var!(name);
    let result = if name.is_empty() {
      write!(f, "<script>")
    } else {
      write!(f, "<fn {}>", name)
    };
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = size_of::<Function>()
      + self.upvalues.capacity() * size_of::<Upvalue>()
      + self.chunk.instructions.instructions.capacity() * size_of::<Instruction>()
      + self.chunk.constants.constants.capacity() * size_of::<Value>()
      + self.chunk.constants.constants.capacity() * size_of::<usize>();
    trace_var!(result);
    trace_exit!();
    result
  }

  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    trace_var!(garbage_collector);
    garbage_collector.mark_object(self.name);
    for &constant in &self.chunk.constants.constants {
      garbage_collector.mark_value(constant);
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
