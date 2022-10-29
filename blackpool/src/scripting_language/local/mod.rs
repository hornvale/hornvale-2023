use crate::scripting_language::token::Token;

/// The `Local` type, used to refer to local variables.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Local {
  /// The name of this local variable.
  pub token: Token,
  /// The scope depth of this variable.
  pub depth: i32,
  /// Whether this is captured or not.
  pub is_captured: bool,
}

impl Local {
  /// Constructor.
  #[named]
  pub fn new(token: Token, depth: i32) -> Self {
    trace_enter!();
    trace_var!(token);
    trace_var!(depth);
    let is_captured = false;
    trace_var!(is_captured);
    let result = Local {
      token,
      depth,
      is_captured,
    };
    trace_var!(result);
    trace_exit!();
    result
  }
}
