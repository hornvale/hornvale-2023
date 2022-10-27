use crate::scripting_language::token::Token;

/// The `Local` type, used to refer to local variables.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Local {
  /// The name of this local variable.
  pub token: Token,
  /// The scope depth of this variable.
  pub depth: i32,
}

impl Local {
  /// Constructor.
  #[named]
  pub fn new(token: Token, depth: i32) -> Self {
    trace_enter!();
    trace_var!(token);
    trace_var!(depth);
    let result = Local { token, depth };
    trace_var!(result);
    trace_exit!();
    result
  }
}
