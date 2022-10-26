use crate::scripting_language::chunk::Chunk;
use crate::scripting_language::garbage_collection::collector::Collector as GarbageCollector;
use crate::scripting_language::parser::Parser;
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
  pub fn compile<'source>(
    &mut self,
    source: &'source str,
    chunk: &mut Chunk,
    garbage_collector: &mut GarbageCollector,
  ) -> Result<(), Error> {
    trace_enter!();
    trace_var!(source);
    trace_var!(chunk);
    let scanner = Scanner::new(source);
    trace_var!(scanner);
    let mut parser = Parser::new(scanner, garbage_collector);
    trace_var!(parser);
    parser.advance()?;
    while !parser.r#match(TokenType::Eof)? {
      parser.parse_declaration(chunk)?;
    }
    self.finalize(&mut parser, chunk)?;
    trace_exit!();
    Ok(())
  }

  /// Conclude.
  #[named]
  pub fn finalize(&mut self, parser: &mut Parser, chunk: &mut Chunk) -> Result<(), Error> {
    trace_enter!();
    trace_var!(parser);
    trace_var!(chunk);
    parser.emit_return(chunk)?;
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
