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

  pub fn new(closure: Reference<Closure>, index: usize) -> Self {
    let instruction_pointer = 0;

    CallFrame {
      closure,
      instruction_pointer,
      index,
    }
  }
}
