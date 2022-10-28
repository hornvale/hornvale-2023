use crate::scripting_language::closure::Closure;
use crate::scripting_language::garbage_collection::reference::Reference;

/// The `CallFrame` type.
#[derive(Clone, Copy, Debug)]
pub struct CallFrame {
  /// The closure this callframe encapsulates.
  pub closure: Reference<Closure>,
  /// Points at the next instruction to read.
  pub instruction_pointer: usize,
  /// Points at our window in the stack.
  pub index: usize,
}

impl CallFrame {
  /// Constructor.
  #[named]
  pub fn new(closure: Reference<Closure>, index: usize) -> Self {
    trace_enter!();
    trace_var!(closure);
    trace_var!(index);
    let instruction_pointer = 0;
    trace_var!(instruction_pointer);
    let result = CallFrame {
      closure,
      instruction_pointer,
      index,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
