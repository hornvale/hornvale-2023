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
  /// Compile some source.
  #[named]
  pub fn compile<'source>(&mut self, source: &'source str, program: &mut Program) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    let scanner = Scanner::new(source);
    trace_var!(scanner);
    let mut parser = Parser::new(scanner);
    trace_var!(parser);
    parser.advance()?;
    parser.parse_expression(program)?;
    parser.consume(TokenType::Eof, "expected end of expression")?;
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
