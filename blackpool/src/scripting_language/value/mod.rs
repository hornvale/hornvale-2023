use std::any::Any;
use std::fmt::{Formatter, Result as FmtResult};

use crate::scripting_language::closure::Closure;
use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::garbage_collection::trace::Trace;
use crate::scripting_language::native_function::NativeFunction;

/// The `Value` enum.
#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Value {
  /// Boolean.
  Boolean(bool),
  /// Closure!
  Closure(Reference<Closure>),
  /// Function!
  Function(Reference<Function>),
  /// A native function.
  NativeFunction(NativeFunction),
  /// Number is a double.
  Number(f64),
  /// String.
  String(Reference<String>),
  /// Nil.
  Nil,
}

impl Value {
  /// Is this "falsey" or not?
  #[named]
  #[inline]
  pub fn is_falsey(&self) -> bool {
    trace_enter!();
    use Value::*;
    let result = match &self {
      Nil => true,
      Boolean(value) => !value,
      _ => false,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}

impl Trace for Value {
  /// Format.
  #[named]
  fn format(&self, f: &mut Formatter, garbage_collector: &GarbageCollector) -> FmtResult {
    trace_enter!();
    trace_var!(garbage_collector);
    use Value::*;
    let result = match self {
      Boolean(value) => write!(f, "{}", value),
      // BoundMethod(value) => gc.deref(*value).format(f, garbage_collector),
      // Class(value) => garbage_collector.deref(*value).format(f, garbage_collector),
      Closure(value) => garbage_collector.deref(*value).format(f, garbage_collector),
      Function(value) => garbage_collector.deref(*value).format(f, garbage_collector),
      // Instance(value) => garbage_collector.deref(*value).format(f, garbage_collector),
      NativeFunction(_) => write!(f, "<native fn>"),
      Nil => write!(f, "nil"),
      Number(value) => write!(f, "{}", value),
      String(value) => garbage_collector.deref(*value).format(f, garbage_collector),
    };
    trace_var!(result);
    trace_exit!();
    result
  }
  /// Get the size.
  #[named]
  fn get_size(&self) -> usize {
    trace_enter!();
    let result = 0;
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Trace!
  #[named]
  fn trace(&self, garbage_collector: &mut GarbageCollector) {
    trace_enter!();
    trace_var!(garbage_collector);
    match self {
      // Value::BoundMethod(value) => garbage_collector.mark_object(*value),
      // Value::Class(value) => garbage_collector.mark_object(*value),
      Value::Closure(value) => garbage_collector.mark_object(*value),
      Value::Function(value) => garbage_collector.mark_object(*value),
      // Value::Instance(value) => garbage_collector.mark_object(*value),
      Value::String(value) => garbage_collector.mark_object(*value),
      _ => (),
    };
    trace_exit!();
  }
  /// Downcast.
  #[named]
  fn as_any(&self) -> &dyn Any {
    trace_enter!();
    panic!("value should not be allocated");
  }
  /// Downcast.
  #[named]
  fn as_any_mut(&mut self) -> &mut dyn Any {
    trace_enter!();
    panic!("value should not be allocated");
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_math() {
    init();
    trace_enter!();
    use crate::scripting_language::value::Value::*;
    test_instructions!([Negate], [Number(53.0)] => [Number(-53.0)]);
    test_instructions!([Negate], [Number(-53.0)] => [Number(53.0)]);
    // The order of the following binary operations can be a bit counterintuitive.
    // With a binary operation, this follows the pattern:
    //
    // test_instructions!([operation], [a, b] => [b operation a]);
    //
    // So for subtraction:
    //
    // test_instructions!([-], [a, b] => [b - a]);
    //
    // As a concrete example:
    //
    // test_instructions!([-], [10, 1] => [1 - 10 = -9]);
    test_instructions!([Add], [Number(-53.0), Number(4.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(4.0), Number(-53.0)] => [Number(-49.0)]);
    test_instructions!([Add], [Number(-3.0), Number(4.0)] => [Number(1.0)]);
    test_instructions!([Add], [Number(4.0), Number(3.0)] => [Number(7.0)]);
    test_instructions!([Subtract], [Number(-53.0), Number(4.0)] => [Number(57.0)]);
    test_instructions!([Subtract], [Number(4.0), Number(-53.0)] => [Number(-57.0)]);
    test_instructions!([Subtract], [Number(-3.0), Number(4.0)] => [Number(7.0)]);
    test_instructions!([Subtract], [Number(4.0), Number(3.0)] => [Number(-1.0)]);
    test_instructions!([Multiply], [Number(4.0), Number(5.0)] => [Number(20.0)]);
    test_instructions!([Multiply], [Number(2.0), Number(-5.0)] => [Number(-10.0)]);
    test_instructions!([Divide], [Number(4.0), Number(5.0)] => [Number(1.25)]);
    test_instructions!([Divide], [Number(32.0), Number(-128.0)] => [Number(-4.0)]);
    test_instructions!([Add, Divide, Negate], [Number(1.2), Number(3.4), Number(5.6)] => [Number(-1.2173)]);
    trace_exit!();
  }
}
