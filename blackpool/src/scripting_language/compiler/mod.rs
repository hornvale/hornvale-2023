use crate::scripting_language::function::upvalue::Upvalue as FunctionUpvalue;
use crate::scripting_language::function::Function;
use crate::scripting_language::garbage_collection::reference::Reference;
use crate::scripting_language::local::Local;
use crate::scripting_language::token::r#type::Type as TokenType;
use crate::scripting_language::token::Token;

pub mod error;
pub mod function_type;
use function_type::FunctionType;

/// The `Compiler` type.
#[derive(Clone, Debug)]
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
    let mut locals = Vec::new();
    trace_var!(locals);
    let depth = 0;
    trace_var!(depth);
    let function = Function::new(function_name);
    trace_var!(function);
    let enclosing = None;
    trace_var!(enclosing);
    let token = match function_type {
      FunctionType::Method | FunctionType::Initializer => Token::synthesize(TokenType::This),
      _ => Token::synthesize(TokenType::EmptyString),
    };
    trace_var!(token);
    locals.push(Local::new(token, 0));
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
  pub fn resolve_local(&mut self, source: &str, token: Token, errors: &mut Vec<&'static str>) -> Option<u16> {
    trace_enter!();
    trace_var!(token);
    let token_name = &source[token.start..(token.start + token.length)];
    trace_var!(token_name);
    for (i, local) in self.locals.iter().enumerate().rev() {
      let local_name = &source[local.token.start..(local.token.start + local.token.length)];
      trace_var!(local_name);
      if token_name == local_name {
        if local.depth == -1 {
          errors.push("Can't read local variable in its own initializer.");
        }
        return Some(i as u16);
      }
    }
    None
  }

  /// Resolve an upvalue.
  #[named]
  pub fn resolve_upvalue(&mut self, source: &str, token: Token, errors: &mut Vec<&'static str>) -> Option<u16> {
    trace_enter!();
    trace_var!(source);
    trace_var!(token);
    if let Some(enclosing) = self.enclosing.as_mut() {
      if let Some(index) = enclosing.resolve_local(source, token, errors) {
        enclosing.locals[index as usize].is_captured = true;
        return Some(self.add_upvalue(index, true, errors));
      }
      if let Some(index) = enclosing.resolve_upvalue(source, token, errors) {
        return Some(self.add_upvalue(index, false, errors));
      }
    }
    trace_exit!();
    None
  }

  /// Create a new upvalue.
  #[named]
  pub fn add_upvalue(&mut self, index: u16, is_local: bool, errors: &mut Vec<&'static str>) -> u16 {
    trace_enter!();
    trace_var!(index);
    trace_var!(is_local);
    trace_var!(errors);
    for (i, upvalue) in self.function.upvalues.iter().enumerate() {
      if upvalue.index == index && upvalue.is_local == is_local {
        return i as u16;
      }
    }
    let result = self.function.upvalues.len() as u16;
    let upvalue = FunctionUpvalue { index, is_local };
    self.function.upvalues.push(upvalue);
    trace_var!(result);
    trace_exit!();
    result
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
