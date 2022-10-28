use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::local::Local;
use crate::scripting_language::token::Token;

pub mod error;
use error::Error;
pub mod function_type;
use function_type::FunctionType;

/// The `Compiler` type.
#[derive(Clone, Debug, PartialEq)]
pub struct Compiler {
  /// The compiler enclosing us.
  pub enclosing: Option<Box<Compiler>>,
  /// The function we're compiling.
  pub function: Function,
  /// The type of function we're compiling.
  pub function_type: FunctionType,
  /// Local variables.
  pub locals: Vec<Local>,
  /// Scope depth.
  pub depth: i32,
}

impl Compiler {
  /// Constructor.
  #[named]
  pub fn new(function_name: Reference<String>, function_type: FunctionType) -> Self {
    trace_enter!();
    let locals = Vec::new();
    trace_var!(locals);
    let depth = 0;
    trace_var!(depth);
    let function = Function::new(function_name);
    trace_var!(function);
    let enclosing = None;
    trace_var!(enclosing);
    let result = Self {
      enclosing,
      locals,
      depth,
      function,
      function_type,
    };
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

  use crate::test::*;

  #[named]
  #[test]
  pub fn test_compiler() {
    init();
    trace_enter!();
    trace_exit!();
  }
}
