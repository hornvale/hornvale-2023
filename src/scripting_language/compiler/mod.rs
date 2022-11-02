use crate::scripting_language::function::upvalue::Upvalue as FunctionUpvalue;
use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::local::Local;
use crate::scripting_language::token::Token;

pub mod error;
pub mod function_type;
use function_type::FunctionType;

/// The `Compiler` type.
#[derive(Clone, Debug)]
pub struct Compiler<'source> {
  /// The compiler enclosing us.
  pub enclosing: Option<Box<Compiler<'source>>>,
  /// The function we're compiling.
  pub function: Function,
  /// The type of function we're compiling.
  pub function_type: FunctionType,
  /// Local variables.
  pub locals: Vec<Local<'source>>,
  /// Scope depth.
  pub depth: i32,
}

impl<'source> Compiler<'source> {
  /// Constructor.

  pub fn new(function_name: Reference<String>, function_type: FunctionType) -> Self {
    let mut locals = Vec::new();

    let depth = 0;

    let function = Function::new(function_name);

    let enclosing = None;

    let token = match function_type {
      FunctionType::Method | FunctionType::Initializer => Token::synthesize("this"),
      _ => Token::synthesize(""),
    };

    locals.push(Local::new(token, 0));

    Self {
      enclosing,
      locals,
      depth,
      function,
      function_type,
    }
  }

  /// Does a variable with this name exist already in this scope?

  pub fn has_local(&self, token: &Token<'source>) -> bool {
    let token_name = token.lexeme;

    for local in self.locals.iter().rev() {
      // We've entered a new scope, so we're safe.
      if local.depth != -1 && local.depth < self.depth {
        return false;
      }
      if token_name == local.token.lexeme {
        return true;
      }
    }

    false
  }

  /// Resolve a local variable.

  pub fn resolve_local(&mut self, token: Token, errors: &mut Vec<&'static str>) -> Option<u16> {
    let token_name = token.lexeme;

    for (i, local) in self.locals.iter().enumerate().rev() {
      if token_name == local.token.lexeme {
        if local.depth == -1 {
          errors.push("Can't read local variable in its own initializer.");
        }
        return Some(i as u16);
      }
    }
    None
  }

  /// Resolve an upvalue.

  pub fn resolve_upvalue(&mut self, token: Token, errors: &mut Vec<&'static str>) -> Option<u16> {
    if let Some(enclosing) = self.enclosing.as_mut() {
      if let Some(index) = enclosing.resolve_local(token, errors) {
        enclosing.locals[index as usize].is_captured = true;
        return Some(self.add_upvalue(index, true, errors));
      }
      if let Some(index) = enclosing.resolve_upvalue(token, errors) {
        return Some(self.add_upvalue(index, false, errors));
      }
    }

    None
  }

  /// Create a new upvalue.

  pub fn add_upvalue(&mut self, index: u16, is_local: bool, _errors: &mut [&'static str]) -> u16 {
    for (i, upvalue) in self.function.upvalues.iter().enumerate() {
      if upvalue.index == index && upvalue.is_local == is_local {
        return i as u16;
      }
    }
    let result = self.function.upvalues.len() as u16;
    let upvalue = FunctionUpvalue { index, is_local };
    self.function.upvalues.push(upvalue);

    result
  }
}

#[cfg(test)]
pub mod test {

  use crate::test::*;

  #[test]
  pub fn test_compiler() {
    init();
  }
}
