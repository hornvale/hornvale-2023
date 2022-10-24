use crate::scripting_language::parser::Parser;
use crate::scripting_language::program::Program;
use crate::scripting_language::scanner::Scanner;
use crate::scripting_language::token::r#type::Type as TokenType;

pub mod error;
use error::Error;

/// The `Compiler` type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Display, Hash, PartialEq, Serialize)]
pub struct Compiler {}

impl Compiler {
  /// Constructor.
  #[named]
  pub fn new() -> Self {
    trace_enter!();
    let result = Self {};
    trace_var!(result);
    trace_exit!();
    result
  }

  /// Compile some source.
  #[named]
  pub fn compile(&mut self, source: &String, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    let mut scanner = Scanner::new(source);
    trace_var!(scanner);
    let mut parser = Parser::default();
    trace_var!(parser);
    parser.advance(&mut scanner)?;
    parser.expression()?;
    parser.consume(&mut scanner, TokenType::Eof, "expected end of expression")?;
    self.finalize(&mut parser, program)?;
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn finalize(&mut self, parser: &mut Parser, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(parser);
    trace_var!(program);
    parser.emit_return(program)?;
    trace_exit!();
    Ok(())
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
