use crate::scripting_language::local::Local;
use crate::scripting_language::token::Token;

pub mod error;
use error::Error;

/// The `Compiler` type.
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Compiler {
  /// Local variables.
  pub locals: Vec<Local>,
  /// Scope depth.
  pub depth: i32,
}

impl Compiler {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let locals = Vec::new();
    trace_var!(locals);
    let depth = 0;
    trace_var!(depth);
    let result = Self { locals, depth };
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Does a variable with this name exist already in this scope?
  #[named]
  pub fn has_local(&self, source: &str, token: &Token) -> bool {
    trace_enter!();
    trace_var!(token);
    let token_name = &source[token.start..(token.start + token.length)];
    trace_var!(token_name);
    for local in self.locals.iter().rev() {
      // We've entered a new scope, so we're safe.
      if local.depth != -1 && local.depth < self.depth {
        return false;
      }
      let local_name = &source[local.token.start..(local.token.start + local.token.length)];
      trace_var!(local_name);
      if token_name == local_name {
        return true;
      }
    }
    trace_exit!();
    false
  }

  /// Resolve a local variable.
  #[named]
  pub fn resolve_local(&mut self, source: &str, token: Token) -> Result<Option<u16>, Error> {
    trace_enter!();
    trace_var!(token);
    let token_name = &source[token.start..(token.start + token.length)];
    trace_var!(token_name);
    for (i, local) in self.locals.iter().enumerate().rev() {
      let local_name = &source[local.token.start..(local.token.start + local.token.length)];
      trace_var!(local_name);
      if token_name == local_name {
        if local.depth == -1 {
          return Err(Error::AttemptedToReadVariableInOwnInitializer(Some(token)));
        } else {
          return Ok(Some(i as u16));
        }
      }
    }
    Ok(None)
  }
}

#[cfg(test)]
pub mod test {

  use super::*;
  use crate::test::*;

  #[named]
  #[test]
  pub fn test_compiler() {
    init();
    trace_enter!();
    let compiler = Compiler::default();
    print_var!(compiler);
    trace_exit!();
  }
}
