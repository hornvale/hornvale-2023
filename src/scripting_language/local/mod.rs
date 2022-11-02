use crate::scripting_language::token::Token;

/// The `Local` type, used to refer to local variables.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Local<'source> {
  /// The name of this local variable.
  pub token: Token<'source>,
  /// The scope depth of this variable.
  pub depth: i32,
  /// Whether this is captured or not.
  pub is_captured: bool,
}

impl<'source> Local<'source> {
  /// Constructor.

  pub fn new(token: Token<'source>, depth: i32) -> Self {
    let is_captured = false;

    Local {
      token,
      depth,
      is_captured,
    }
  }
}
